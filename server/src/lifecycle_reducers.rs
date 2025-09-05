use std::time::Duration;

use log::warn;
use spacetimedb::{ReducerContext, Table};

use crate::{
    schedules::create_games::{create_games_schedule, CreateGameSchedule},
    types::{
        client::Client,
        player::{Player, PlayerState},
    },
};

#[spacetimedb::reducer(client_connected)]
/// Inserts a client row without a player.
pub(crate) fn on_client_connected(ctx: &ReducerContext) {
    Client::insert_sender(ctx);
}

#[spacetimedb::reducer(client_disconnected)]
pub(crate) fn on_client_disconnected(ctx: &ReducerContext) {
    let client = Client::get_sender(ctx);

    if !Client::delete_sender(ctx) {
        warn!("Client was not found on diconnect!");
    }

    if client.player_steam_id.is_none() {
        return;
    }

    let mut player = match Player::get(ctx, client.player_steam_id.unwrap()) {
        Some(p) => p,
        None => unreachable!("There should always be a player here."),
    };

    player.state = PlayerState::Offline;
    Player::update(ctx, player);
}

#[spacetimedb::reducer(init)]
pub(crate) fn init(ctx: &ReducerContext) {
    ctx.db.create_games_schedule().insert(CreateGameSchedule {
        scheduled_id: 0,
        scheduled_at: Duration::from_secs(2).into(),
    });
}
