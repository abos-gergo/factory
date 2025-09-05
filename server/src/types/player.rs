use crate::types::client::Client;
use log::info;
use spacetimedb::{ReducerContext, ReducerResult, SpacetimeType, Table};

#[derive(Debug)]
#[spacetimedb::table(name = player, public)]
pub(crate) struct Player {
    #[primary_key]
    pub(crate) steam_id: u64,
    pub(crate) name: String,
    pub(crate) state: PlayerState,
}
impl Player {
    pub(crate) fn new(steam_id: u64) -> Self {
        Self {
            steam_id,
            name: "".into(),
            state: PlayerState::Idle,
        }
    }
    pub(crate) fn get(ctx: &ReducerContext, steam_id: u64) -> Option<Self> {
        ctx.db.player().steam_id().find(steam_id)
    }
    pub(crate) fn authenticate(ctx: &ReducerContext) -> Result<Self, &'static str> {
        let client = Client::get_sender(ctx);

        let steam_id = client.player_steam_id.ok_or("You are not authenticated")?;

        let player = Player::get(ctx, steam_id).ok_or("This player is not found")?;

        Ok(player)
    }

    pub(crate) fn insert(ctx: &ReducerContext, player: Self) -> Self {
        let p = ctx.db.player().insert(player);
        info!("Inserted into Player:\n\t{:?}", p);
        p
    }
    pub(crate) fn update(ctx: &ReducerContext, player: Self) -> Self {
        let p = ctx.db.player().steam_id().update(player);
        info!("Updated Player from to:\n\t{:?}", p);
        p
    }

    pub(crate) fn in_state<'a>(
        ctx: &'a ReducerContext,
        state: PlayerState,
    ) -> impl Iterator<Item = Self> + 'a {
        ctx.db.player().iter().filter(move |p| p.state == state)
    }
}

#[derive(SpacetimeType, Debug, PartialEq)]
pub(crate) enum PlayerState {
    Offline,
    Idle,
    SearchingForGame,
    InGame(u64),
}

#[spacetimedb::reducer]
pub(crate) fn set_name(ctx: &ReducerContext, name: String) -> ReducerResult {
    let steam_id = Client::get_sender(ctx).player_steam_id;

    if steam_id.is_none() {
        return Err("You are not authenticated".into());
    }
    let steam_id = steam_id.unwrap();

    let mut player = Player::get(ctx, steam_id).expect("There should always be a player here.");

    if !player.name.is_empty() {
        return Err("You've already set your name".into());
    }

    player.name = name;
    Player::update(ctx, player);

    Ok(())
}

#[spacetimedb::reducer]
/// Sets the Player's state to SearchingForGame
/// If there are more players searching for game than `GAME_SIZE``, create a new game.
pub(crate) fn enter_queue(ctx: &ReducerContext) -> Result<(), String> {
    let mut player = Player::authenticate(ctx)?;

    if player.state != PlayerState::Idle {
        return Err("You can enter queue in Idle state".to_string());
    }

    info!("Player {} entered the queue", player.name.clone());
    player.state = PlayerState::SearchingForGame;
    Player::update(ctx, player);

    Ok(())
}

#[spacetimedb::reducer]
fn exit_queue(ctx: &ReducerContext) -> Result<(), String> {
    let mut player = Player::authenticate(ctx)?;

    if player.state != PlayerState::SearchingForGame {
        return Err("You can only cancel queue in Searching For Game state".to_string());
    }

    info!("Player {} left the queue.", player.name.clone());
    player.state = PlayerState::Idle;
    Player::update(ctx, player);

    Ok(())
}
