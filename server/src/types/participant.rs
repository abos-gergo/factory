use spacetimedb::{ReducerContext, Table};

use crate::types::grid::Grid;

#[derive(Debug)]
#[spacetimedb::table(name = participant, public)]
pub(crate) struct Participant {
    #[primary_key]
    pub(crate) steam_id: u64,
    pub(crate) game_id: u64,
    pub(crate) grid: Grid,
}
impl Participant {
    pub(crate) fn new(steam_id: u64, game_id: u64) -> Self {
        Self {
            steam_id,
            game_id,
            grid: Grid::new(),
        }
    }
    pub(crate) fn get(ctx: &ReducerContext, steam_id: u64) -> Option<Self> {
        ctx.db.participant().steam_id().find(steam_id)
    }
    pub(crate) fn insert(ctx: &ReducerContext, participant: Self) -> Self {
        ctx.db.participant().insert(participant)
    }
    pub(crate) fn in_game<'a>(
        ctx: &'a ReducerContext,
        game_id: u64,
    ) -> impl Iterator<Item = Self> + 'a {
        ctx.db
            .participant()
            .iter()
            .filter(move |p| p.game_id == game_id)
    }
}
