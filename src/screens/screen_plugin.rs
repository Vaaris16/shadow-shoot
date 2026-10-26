use bevy::prelude::*;

use crate::screens::{
    endgame::endgame_plugin::EndgamePlugin, game::game_plugin::GamePlugin,
    splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashScreenPlugin, GamePlugin, EndgamePlugin));
    }
}
