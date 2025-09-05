use crate::stdb::{
    AuthRequest, AuthRequestTableAccess, DbConnection, EventContext, SubscriptionHandle,
    auth_server_response,
};
use spacetimedb_sdk::{DbContext, Table};
use std::net::SocketAddrV4;
use std::str::FromStr;
use steamworks::{AuthSessionError, Server, ServerMode, SteamId};
use tracing::{error, info};

mod stdb;

fn main() {
    tracing_subscriber::fmt::init();

    let server = create_steam_server();

    let stdb = connect_to_spacetimedb("http://localhost:3000", "fac");
    subscribe_to_tables(&stdb);
    stdb.db()
        .auth_request()
        .on_insert(move |ctx, auth_request| on_auth_request_insteted(ctx, auth_request, &server));

    stdb.run_threaded().join().unwrap();
}

fn create_steam_server() -> Server {
    let bind_address =
        SocketAddrV4::from_str("0.0.0.0:54321").expect("Could not parse bind address");

    // Client::init is not required with Server::Init as long as you set the app id via steam_appid.txt or env var
    unsafe {
        std::env::set_var("SteamAppId", "480");
    }
    let (server, _) = steamworks::Server::init(
        *bind_address.ip(), // in reality, this should be the external ip of the server
        bind_address.port() - 1,
        bind_address.port(), // For some games, this port is actually the "main" port (and the game_port is unused)
        ServerMode::AuthenticationAndSecure,
        "123456",
    )
    .expect("Could not register server with Steam Master server list");

    server.set_server_name("Cardgame steam auth server");
    server.set_dedicated_server(true);
    server.log_on_anonymous();

    server
}

fn connect_to_spacetimedb(host_uri: &'static str, module_name: &'static str) -> DbConnection {
    DbConnection::builder()
        .on_connect(move |_conn, _identity, _token| {
            info!("Connected to SpacetimeDB at {}: {}", host_uri, module_name)
        })
        .on_connect_error(move |_ectx, e| {
            error!(
                "Failed to connect to SpacetimeDB server at {} - {}: {}",
                host_uri, module_name, e
            );
        })
        .with_module_name(module_name)
        .with_uri(host_uri)
        .build()
        .expect("Failed to connect")
}

fn subscribe_to_tables(ctx: &DbConnection) -> SubscriptionHandle {
    ctx.subscription_builder()
        .on_applied(|_ctx| info!("Successfully subscribed to AuthRequests"))
        .on_error(|_ctx, e| error!("Could not subscribe to AuthRequests: {}", e))
        .subscribe(["SELECT * FROM auth_request"])
}

fn on_auth_request_insteted(ctx: &EventContext, auth_request: &AuthRequest, server: &Server) {
    let res = server.begin_authentication_session(
        SteamId::from_raw(auth_request.steam_id),
        &auth_request.ticket,
    );

    let err_msg = if let Err(err) = res {
        match err {
            AuthSessionError::InvalidTicket => "Invalid Ticket",
            AuthSessionError::DuplicateRequest => "Duplicate Request",
            AuthSessionError::InvalidVersion => "Invalid Version",
            AuthSessionError::GameMismatch => "Game Mismatch",
            AuthSessionError::ExpiredTicket => "Expired Ticket",
        }
    } else {
        ""
    };

    ctx.reducers
        .auth_server_response(
            auth_request.steam_id,
            auth_request.identity,
            err_msg.to_string(),
        )
        .unwrap();

    info!("Authenticated user with steamId: {}", auth_request.steam_id);
}
