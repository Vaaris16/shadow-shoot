use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::ui::{
        player_lifes::{PlayerLifes, spawn_player_ui},
        score::{Score, ScoreStepTimer, reset_score},
    },
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerLifes::default())
            .insert_resource(Score::default())
            .insert_resource(ScoreStepTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::SplashScreen), reset_score)
            .add_systems(OnEnter(GameState::Game), spawn_player_ui);
    }
}
