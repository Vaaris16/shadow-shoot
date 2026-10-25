use bevy::{prelude::*, window::WindowMode};

use crate::{core::core_plugin::CorePlugin, screens::screen_plugin::ScreenPlugin};

mod core;
mod screens;

pub const GAME_BG_COLOR: Color = Color::hsl(219., 0.40, 0.08);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    SplashScreen,
    #[default]
    Game,
    EndGame,
}

fn main() {
    App::new()
        .add_plugins((CorePlugin, ScreenPlugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Fullscreen Window".into(),
                // Choose BorderlessFullscreen or Fullscreen
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .run();
}
