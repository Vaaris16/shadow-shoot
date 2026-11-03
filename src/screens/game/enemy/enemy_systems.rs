use bevy::{prelude::*, window::PrimaryWindow};

use crate::screens::game::enemy::{
    enemy_component::Enemy, player_enemies::spawn_player_enemy, shadow_enemies::spawn_shadow_enemy,
};

#[derive(Resource)]
pub struct EnemyCount(pub i32);

impl Default for EnemyCount {
    fn default() -> Self {
        Self(3)
    }
}

pub fn maintain_enemies(
    enemies: Query<(), With<Enemy>>,
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_count: Res<EnemyCount>,
) {
    let enemy_num = enemies.iter().len();

    if enemy_num <= enemy_count.0 as usize {
        spawn_player_enemy(&mut commands, &window, &asset_server);
        spawn_shadow_enemy(&mut commands, &window, &asset_server);
    }
}

pub fn move_enemies(enemies: Query<(&Enemy, &mut Transform), With<Enemy>>) {
    for (enemy, mut trans) in enemies {
        trans.translation += enemy.velocity.extend(0.);
    }
}

pub fn despawn_enemies(
    window: Single<&Window, With<PrimaryWindow>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, trans) in enemies {
        if trans.translation.y <= -window.height() / 2. {
            commands.entity(entity).despawn();
        }
    }
}
