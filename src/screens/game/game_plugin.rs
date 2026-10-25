use crate::{
    GameState,
    screens::game::{
        collision::collision_plugin::CollisionPlugin,
        enemy_component::Enemy,
        enemy_systems::*,
        move_player::move_players,
        player::spawn_player,
        score::{Score, ScoreStepTimer, add_score, set_score, spawn_score},
        shadow_player::spawn_shadow_player,
    },
};
use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
struct GameSet;

pub struct GamePlugin;

#[derive(Component, Clone, FromTemplate)]
pub struct MainPlayer;

#[derive(Component, Clone, FromTemplate)]
pub struct ShadowPlayer;

#[derive(Component, Clone, FromTemplate)]
pub struct GameUi;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_plugins(CollisionPlugin)
            .init_resource::<Score>()
            .insert_resource(ScoreStepTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .add_systems(
                OnEnter(GameState::Game),
                (spawn_ui, spawn_player, spawn_shadow_player),
            )
            .add_systems(OnExit(GameState::Game), cleanup_game)
            .add_systems(
                Update,
                (
                    maintain_enemies,
                    move_enemies,
                    despawn_enemies,
                    move_players,
                    set_score,
                    add_score,
                )
                    .in_set(GameSet),
            );
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
        }
        GameUi
        Children [
            @spawn_score()
            --
            @spawn_divider()
        ]
    });
}

fn spawn_divider() -> impl Scene {
    bsn! {
        ImageNode {
            image: "divider.png",
            image_mode: NodeImageMode::Stretch
        }
        Node {
            height: percent(100),
            width: px(5),
        }
    }
}

fn cleanup_game(
    enemies: Query<Entity, With<Enemy>>,
    main_player: Single<Entity, With<MainPlayer>>,
    shadow_player: Single<Entity, With<ShadowPlayer>>,
    game_ui: Single<Entity, With<GameUi>>,
    mut commands: Commands,
) {
    for e in enemies {
        commands.entity(e).despawn();
    }
    commands.entity(*main_player).despawn();
    commands.entity(*shadow_player).despawn();
    commands.entity(*game_ui).despawn();
}
