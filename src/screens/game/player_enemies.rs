use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screens::game::enemy_component::{Enemy, EnemyType};

pub fn spawn_player_enemy(commands: &mut Commands, window: &Window, asset_server: &AssetServer) {
    let mut enemy = Enemy::new(EnemyType::Player);
    enemy.rand_pos(&window);

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(20.)),
            image: asset_server.load(&enemy.enemy_sprite),
            ..Default::default()
        },
        Transform::from_xyz(enemy.pos.x, enemy.pos.y, 0.0),
        enemy,
        Collider::rectangle(20., 20.),
        RigidBody::Kinematic,
        CollisionEventsEnabled,
    ));
}
