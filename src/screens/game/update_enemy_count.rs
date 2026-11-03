use bevy::prelude::*;

use crate::screens::game::{enemy::enemy_systems::EnemyCount, ui::score::Score};

pub fn update_enemy_count(mut enemy_count: ResMut<EnemyCount>, score: Res<Score>) {
    if score.0 >= 90 {
        enemy_count.0 = 10;
    } else if score.0 >= 30 {
        enemy_count.0 = 7;
    } else if score.0 >= 20 {
        enemy_count.0 = 5;
    } else if score.0 >= 10 {
        enemy_count.0 = 4;
    }
}

pub fn reset_enemy_count(mut enemy_count: ResMut<EnemyCount>) {
    enemy_count.0 = EnemyCount::default().0;
}
