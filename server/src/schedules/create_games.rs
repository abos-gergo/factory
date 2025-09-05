use spacetimedb::{ReducerContext, ScheduleAt};

use crate::{
    types::{
        game::Game,
        participant::Participant,
        player::{Player, PlayerState},
    },
    GAME_SIZE,
};

#[spacetimedb::table(name = create_games_schedule, scheduled(create_game))]
pub(crate) struct CreateGameSchedule {
    #[primary_key]
    #[auto_inc]
    pub(crate) scheduled_id: u64,
    pub(crate) scheduled_at: ScheduleAt,
}

#[spacetimedb::reducer]
pub(crate) fn create_game(ctx: &ReducerContext, _: CreateGameSchedule) {
    let player_count = Player::in_state(ctx, PlayerState::SearchingForGame).count();

    let lobbies_to_create = player_count / GAME_SIZE;

    for _ in 0..lobbies_to_create {
        let game = Game::insert(ctx, Game::new());
        let players = Player::in_state(ctx, PlayerState::SearchingForGame).take(GAME_SIZE);

        for mut player in players {
            player.state = PlayerState::InGame(game.id);
            Participant::insert(ctx, Participant::new(player.steam_id, game.id));
            Player::update(ctx, player);
        }
    }
}
