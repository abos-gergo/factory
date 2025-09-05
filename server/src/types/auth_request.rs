use log::info;
use spacetimedb::{Identity, ReducerContext, ReducerResult, Table};

use crate::types::{
    client::Client,
    player::{Player, PlayerState},
};

#[spacetimedb::table(name = auth_request, public)]
pub(crate) struct AuthRequest {
    #[primary_key]
    pub(crate) steam_id: u64,
    #[unique]
    pub(crate) identity: Identity,
    pub(crate) ticket: Vec<u8>,
}

#[spacetimedb::reducer]
pub(crate) fn begin_client_auth(
    ctx: &ReducerContext,
    steam_id: u64,
    ticket: Vec<u8>,
) -> ReducerResult {
    ctx.db.auth_request().insert(AuthRequest {
        steam_id,
        ticket,
        identity: ctx.sender,
    });

    info!(
        "Client with identity {} begun client authentication with SteamID {}",
        ctx.sender, steam_id
    );

    return Ok(());
}

#[spacetimedb::reducer]
pub(crate) fn auth_server_response(
    ctx: &ReducerContext,
    steam_id: u64,
    identity: Identity,
    err_msg: String,
) -> ReducerResult {
    // if ctx.sender != ctx.identity() {
    //     return Err("Clients cannot call this reducer!".into());
    // }

    ctx.db.auth_request().steam_id().delete(steam_id);

    if !err_msg.is_empty() {
        return Err(format!("Authentication for {} failed: {}", steam_id, err_msg).into());
    }

    info!("Authentication for {} was successful", steam_id);

    Client::update(
        ctx,
        Client {
            identity: identity,
            player_steam_id: Some(steam_id),
        },
    );

    // Returning player -> Just set the player to online
    if let Some(mut player) = Player::get(ctx, steam_id) {
        debug_assert!(player.state == PlayerState::Offline);
        player.state = PlayerState::Idle;
        Player::update(ctx, player);
    }
    // New player -> Insert new record. A follow-up reducer will set the player's name.
    else {
        Player::insert(ctx, Player::new(steam_id));
    }

    Ok(())
}
