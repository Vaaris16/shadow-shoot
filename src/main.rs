use avian2d::PhysicsPlugins;
use bevy::{asset::AssetMetaCheck, prelude::*};

use crate::{core::core_plugin::CorePlugin, screens::screen_plugin::ScreenPlugin};

mod core;
mod screens;

pub const GAME_BG_COLOR: Color = Color::hsl(219., 0.40, 0.08);

pub const CYAN: Color = Color::hsl(197.0, 0.89, 0.72);
pub const ORANGE: Color = Color::hsl(18.0, 0.71, 0.66);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    SplashScreen,
    Game,
    #[default]
    EndGame,
}

fn main() {
    App::new()
        .add_plugins((PhysicsPlugins::default(), CorePlugin, ScreenPlugin))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        title: "Shadow Shoot".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .init_state::<GameState>()
        .run();
}
