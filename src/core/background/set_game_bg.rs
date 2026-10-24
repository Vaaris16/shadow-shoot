use bevy::prelude::*;

use crate::GAME_BG_COLOR;

pub fn set_bg(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = GAME_BG_COLOR;
}
