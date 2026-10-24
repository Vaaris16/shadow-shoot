use bevy::prelude::*;

use crate::core::{background::set_game_bg::set_bg, camera::spawn_camera::spawn_camera};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_bg)
            .add_systems(Startup, spawn_camera);
    }
}
