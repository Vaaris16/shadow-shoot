use avian2d::collision::collision_events::CollisionStart;
use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        enemy_component::Enemy,
        game_plugin::{MainPlayer, ShadowPlayer},
    },
};

pub struct CollisionPlugin;

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CollisionSet;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, CollisionSet.run_if(in_state(GameState::Game)))
            .add_systems(Update, collision_object_player.in_set(CollisionSet));
    }
}

fn collision_object_player(
    mut collisions: MessageReader<CollisionStart>,
    main_players: Query<(), With<MainPlayer>>,
    shadow_players: Query<(), With<ShadowPlayer>>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for collision in collisions.read() {
        let (a, b) = (collision.collider1, collision.collider2);

        if (main_players.contains(a) && enemies.contains(b))
            || (main_players.contains(b) && enemies.contains(a))
            || (shadow_players.contains(a) && enemies.contains(b))
            || (shadow_players.contains(b) && enemies.contains(a))
        {
            next_state.set(GameState::EndGame);
        }
    }
}
