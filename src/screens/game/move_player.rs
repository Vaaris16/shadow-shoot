use bevy::{prelude::*, window::PrimaryWindow};

use crate::screens::game::game_plugin::{MainPlayer, ShadowPlayer};

pub fn move_players(
    mut main_player: Single<&mut Transform, (With<MainPlayer>, Without<ShadowPlayer>)>,
    mut shadow_player: Single<&mut Transform, (With<ShadowPlayer>, Without<MainPlayer>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let speed = 500.0;
    let movement_amount = speed * time.delta_secs();

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        main_player.translation.x += movement_amount;
        shadow_player.translation.x += movement_amount;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        main_player.translation.x -= movement_amount;
        shadow_player.translation.x -= movement_amount;
    }

    let half_width = window.width() / 2. - 50.;
    main_player.translation.x = main_player.translation.x.clamp(-half_width, -50.);
    shadow_player.translation.x = shadow_player.translation.x.clamp(50., half_width);
}
