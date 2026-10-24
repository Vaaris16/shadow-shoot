use bevy::prelude::*;

use crate::core::core_plugin::CorePlugin;

mod core;

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
        .add_plugins((DefaultPlugins, CorePlugin))
        .init_state::<GameState>()
        .run();
}
