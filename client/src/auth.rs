use bevy::prelude::*;
use bevy_spacetimedb::StdbConnection;
use bevy_steamworks::{Client, SteamId};

use crate::stdb::{DbConnection, Player, PlayerTableAccess, begin_client_auth};

pub(crate) struct AuthPlugin;
impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoggedInPlayer>()
            .add_systems(Startup, steam_auth)
            .add_systems(Update, manage_logged_in_player);
    }
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub(crate) struct LoggedInPlayer(pub(crate) Option<Player>);

fn manage_logged_in_player(
    stdb: Res<StdbConnection<DbConnection>>,
    steam: Res<Client>,
    mut logged_in_player: ResMut<LoggedInPlayer>,
) {
    let steam_id = steam.user().steam_id().raw();

    let player = stdb.db().player().steam_id().find(&steam_id);

    *logged_in_player = LoggedInPlayer(player);
}

fn steam_auth(steam: Res<bevy_steamworks::Client>, stdb: Res<StdbConnection<DbConnection>>) {
    let (_ticket, ticket_data) = steam
        .user()
        .authentication_session_ticket_with_steam_id(SteamId::from_raw(0));
    let steam_id = steam.user().steam_id().raw();

    stdb.reducers()
        .begin_client_auth(steam_id, ticket_data)
        .unwrap();
}
