use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_spacetimedb::StdbConnection;
use spacetimedb_sdk::Table;

use crate::{
    ScreenState,
    stdb::{DbConnection, PlayerState, PlayerTableAccess, enter_queue, exit_queue, set_name},
};
pub(crate) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            auth_ui.run_if(in_state(ScreenState::AuthenticationInProgress)),
        )
        .add_systems(
            EguiPrimaryContextPass,
            set_name_ui.run_if(in_state(ScreenState::SetName)),
        )
        .add_systems(
            EguiPrimaryContextPass,
            main_menu_ui.run_if(in_state(ScreenState::MainMenu)),
        )
        .add_systems(
            EguiPrimaryContextPass,
            searching_for_game_ui.run_if(in_state(ScreenState::SearchingForGame)),
        );
    }
}

fn auth_ui(mut contexts: EguiContexts) {
    egui::CentralPanel::default().show(contexts.ctx_mut().unwrap(), |ui| {
        ui.vertical_centered(|ui| {
            ui.vertical(|ui| {
                ui.heading("Authentication In Progress");
                ui.spinner();
            });
        });
    });
}

fn set_name_ui(
    mut contexts: EguiContexts,
    stdb: Res<StdbConnection<DbConnection>>,
    mut name: Local<String>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut().unwrap(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical(|ui| {
                ui.heading("Give yourself a name!");
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut *name);
                if ui.button("Save Name").clicked() {
                    stdb.reducers().set_name(name.clone()).unwrap();
                };
            });
        });
    });
}

fn main_menu_ui(mut contexts: EguiContexts, stdb: Res<StdbConnection<DbConnection>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut().unwrap(), |ui| {
        ui.vertical_centered(|ui| {
            ui.vertical(|ui| {
                ui.heading("Main Menu");
                ui.add_space(20.0);
            });

            if ui.button("Find Match").clicked() {
                let _ = stdb.reducers().enter_queue();
            }
        });
    });
}

fn searching_for_game_ui(mut contexts: EguiContexts, stdb: Res<StdbConnection<DbConnection>>) {
    let player_in_queue_count = stdb
        .db()
        .player()
        .iter()
        .filter(|p| p.state == PlayerState::SearchingForGame)
        .count();

    egui::CentralPanel::default().show(contexts.ctx_mut().unwrap(), |ui| {
        ui.vertical_centered(|ui| {
            ui.vertical(|ui| {
                ui.heading("Searching for a Game");
                ui.label(format!("Players in queue: {}", player_in_queue_count));
                ui.spinner();
                if ui.button("Cancel").clicked() {
                    let _ = stdb.reducers().exit_queue();
                };
            });
        });
    });
}
