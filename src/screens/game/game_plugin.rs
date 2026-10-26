use std::time::Duration;

use crate::{
    GameState,
    screens::game::{
        collision::collision_plugin::CollisionPlugin,
        enemy_component::Enemy,
        enemy_systems::*,
        move_player::move_players,
        player::spawn_player,
        player_lifes::{PlayerLifes, reset_lifes, spawn_player_ui},
        score::{Score, ScoreStepTimer, add_score, reset_score, set_score, spawn_score},
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

#[derive(Component)]
pub struct Immune {
    immune_timer: Timer,
    blink_timer: Timer,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_plugins(CollisionPlugin)
            .insert_resource(Score::default())
            .insert_resource(ScoreStepTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .insert_resource(PlayerLifes::default())
            .add_systems(
                OnEnter(GameState::Game),
                (spawn_ui, spawn_player, spawn_shadow_player, spawn_player_ui),
            )
            .add_systems(OnExit(GameState::Game), cleanup_game)
            .add_systems(
                Update,
                (
                    fade_player_immune,
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
    score: ResMut<Score>,
    player_lifes: ResMut<PlayerLifes>,
) {
    for e in enemies {
        commands.entity(e).despawn();
    }
    commands.entity(*main_player).despawn();
    commands.entity(*shadow_player).despawn();
    commands.entity(*game_ui).despawn();

    reset_score(score);
    reset_lifes(player_lifes);
}

pub fn set_players_immune(commands: &mut Commands, player: &Entity) {
    commands.entity(*player).insert(Immune {
        immune_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        blink_timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
    });
}

fn fade_player_immune(
    time: Res<Time>,
    mut players: Query<(&mut Sprite, Entity, &mut Immune), With<Immune>>,
    mut commands: Commands,
) {
    for (mut sprite, entity, mut immune) in players {
        immune.immune_timer.tick(time.delta());
        immune.blink_timer.tick(time.delta());

        let alpha = sprite.color.alpha();

        if immune.blink_timer.just_finished() {
            if alpha > 0.5 {
                sprite.color.set_alpha(0.15);
            } else {
                sprite.color.set_alpha(1.0);
            }
        }
        if immune.immune_timer.just_finished() {
            commands.entity(entity).remove::<Immune>();
        }
    }
}
