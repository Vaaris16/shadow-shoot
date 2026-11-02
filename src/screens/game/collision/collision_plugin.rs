use avian2d::collision::collision_events::CollisionStart;
use bevy::prelude::*;

use crate::{
    GameState,
    screens::game::{
        enemy::enemy_component::Enemy,
        game_plugin::{Immune, MainPlayer, ShadowPlayer, set_players_immune},
        ui::player_lifes::{PlayerLifeIndicator, PlayerLifes, decrement_player_life},
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
    main_players: Single<(Entity, Has<Immune>), With<MainPlayer>>,
    shadow_players: Single<(Entity, Has<Immune>), With<ShadowPlayer>>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_lifes: ResMut<PlayerLifes>,
    player_lifes_indicators: Query<(Entity, &PlayerLifeIndicator), With<PlayerLifeIndicator>>,
    mut commands: Commands,
) {
    for collision in collisions.read() {
        let (a, b) = (collision.collider1, collision.collider2);

        if (main_players.0 == a && enemies.contains(b))
            || (main_players.0 == b && enemies.contains(a))
            || (shadow_players.0 == a && enemies.contains(b))
            || (shadow_players.0 == b && enemies.contains(a))
        {
            if !shadow_players.1 || !main_players.1 {
                decrement_player_life(
                    &mut player_lifes,
                    player_lifes_indicators,
                    &mut next_state,
                    &mut commands,
                );
                set_players_immune(&mut commands, &main_players.0);
                set_players_immune(&mut commands, &shadow_players.0);
            }
        }
    }
}
