use bevy::prelude::*;

use crate::{GameState, screens::game::player_lifes};

#[derive(Resource)]
pub struct PlayerLifes(pub i32);

#[derive(Component)]
pub struct PlayerLifeIndicator {
    index: i32,
}

const ONE_THIRD: f64 = 100. / 3.;

impl Default for PlayerLifes {
    fn default() -> Self {
        Self(3)
    }
}

pub fn spawn_player_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn_scene(bsn! {
            Node {
                width: percent(20),
                height: percent(10),
                position_type: PositionType::Absolute,
                top: px(20),
                left: px(20),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                row_gap: px(20),
                padding: UiRect::all(px(20)),
            }
        })
        .with_children(|parent| {
            for i in 1..4 {
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("player sprites/player_1_sprite.png"),
                        ..Default::default()
                    },
                    PlayerLifeIndicator { index: i },
                    Node {
                        width: percent(ONE_THIRD),
                        height: percent(100),
                        ..Default::default()
                    },
                    children![(
                        Text::new(i.to_string()),
                        TextFont {
                            font_size: px(20).into(),
                            ..Default::default()
                        },
                        TextColor(Color::BLACK),
                    )],
                ));
            }
        });
}

pub fn decrement_player_life(
    player_lifes: &mut PlayerLifes,
    player_lifes_indicators: Query<(Entity, &PlayerLifeIndicator), With<PlayerLifeIndicator>>,
    next_state: &mut NextState<GameState>,
    commands: &mut Commands,
) {
    player_lifes.0 -= 1;

    if player_lifes.0 == 0 {
        next_state.set(GameState::EndGame);
    }

    for (entity, indicator) in player_lifes_indicators {
        if indicator.index > player_lifes.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn reset_lifes(mut player_lifes: ResMut<PlayerLifes>) {
    player_lifes.0 = 3;
}
