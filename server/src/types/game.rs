use log::info;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

#[derive(Debug)]
#[spacetimedb::table(name = game, public)]
pub(crate) struct Game {
    #[primary_key]
    #[auto_inc]
    pub(crate) id: u64,
    pub(crate) state: GameState,
}
impl Game {
    pub(crate) fn new() -> Self {
        Self {
            // Will get replaced by actual id value when inserting into db
            id: 0,
            state: GameState::Todo,
        }
    }
    pub(crate) fn get(ctx: &ReducerContext, game_id: u64) -> Option<Self> {
        ctx.db.game().iter().find(|g| g.id == game_id)
    }
    pub(crate) fn insert(ctx: &ReducerContext, game: Self) -> Self {
        let g = ctx.db.game().insert(game);
        info!("Inserted into Game:\n\t{:?}", g);
        g
    }
    pub(crate) fn update(ctx: &ReducerContext, game: Self) -> Self {
        let g = ctx.db.game().id().update(game);
        info!("Updated Game:\n\t{:?}", g);
        g
    }
}

#[derive(Debug, SpacetimeType, PartialEq)]
pub(crate) enum GameState {
    Todo,
}
