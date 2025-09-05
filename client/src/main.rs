use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_spacetimedb::{StdbConnection, StdbPlugin};
use bevy_steamworks::SteamworksPlugin;

use crate::{
    auth::{AuthPlugin, LoggedInPlayer},
    stdb::{ClientTableAccess, DbConnection, PlayerState, PlayerTableAccess, RemoteTables}, ui::UIPlugin,
};

mod auth;
mod stdb;
mod ui;

fn main() {
    App::new()
        .add_plugins(SteamworksPlugin::init_app(480).unwrap())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(
            StdbPlugin::default()
                .with_uri("http://localhost:3000")
                .with_module_name("fac")
                .with_run_fn(DbConnection::run_threaded)
                .add_table(RemoteTables::client)
                .add_table(RemoteTables::player),
        )
        // Custom plugins
        .add_plugins(AuthPlugin)
        .add_plugins(UIPlugin)
        .init_state::<ScreenState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, subscribe_to_tables)
        .add_systems(Update, manage_screen_state)
        .run();
}

fn subscribe_to_tables(
    steam: Res<bevy_steamworks::Client>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    let steam_id = steam.user().steam_id().raw();
    let sql = format!("SELECT * FROM player WHERE steam_id = '{}'", steam_id);
    stdb.subscription_builder()
        .on_applied(|_ctx| info!("Successfully subscribed to player"))
        .on_error(|_ctx, e| error!("Could not subscribe to player: {}", e))
        .subscribe(sql);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn manage_screen_state(
    mut next_screen_state: ResMut<NextState<ScreenState>>,
    logged_in_player: Res<LoggedInPlayer>,
) {
    if let Some(player) = logged_in_player.clone() {
        if player.state == PlayerState::Offline {
            return;
        }
        if player.name.is_empty() {
            next_screen_state.set(ScreenState::SetName);
            return;
        }
        if player.state == PlayerState::Idle {
            next_screen_state.set(ScreenState::MainMenu);
            return;
        }
        if player.state == PlayerState::SearchingForGame {
            next_screen_state.set(ScreenState::SearchingForGame);
            return;
        }
        if let PlayerState::InGame(game_id) = player.state {
            next_screen_state.set(ScreenState::InGame(game_id));
            return;
        }
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
enum ScreenState {
    #[default]
    AuthenticationInProgress,
    SetName,
    MainMenu,
    SearchingForGame,
    InGame(u64),
}
