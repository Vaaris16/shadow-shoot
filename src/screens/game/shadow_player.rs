use bevy::{prelude::*, window::PrimaryWindow};

use crate::screens::game::game_plugin::ShadowPlayer;

pub fn spawn_shadow_player(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
    let x = window.width() / 4.;
    let y = -window.height() / 2. + 75.;
    commands.spawn_scene(bsn! {
        Sprite {
            image: "player sprites/player_2_sprite.png",
            custom_size: Vec2::splat(50.),
        }
        Transform::from_xyz(x, y, 0.)
        ShadowPlayer
    });
}
