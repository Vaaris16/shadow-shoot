use avian2d::PhysicsPlugins;
use bevy::{prelude::*, window::WindowMode};

use crate::{core::core_plugin::CorePlugin, screens::screen_plugin::ScreenPlugin};

mod core;
mod screens;

pub const GAME_BG_COLOR: Color = Color::hsl(219., 0.40, 0.08);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    SplashScreen,
    Game,
    EndGame,
}

fn main() {
    App::new()
        .add_plugins((PhysicsPlugins::default(), CorePlugin, ScreenPlugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "app".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .run();
}
