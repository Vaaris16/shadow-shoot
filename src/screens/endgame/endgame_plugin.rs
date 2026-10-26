use std::f32::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_4};

use bevy::{
    prelude::*,
    text::FontSourceTemplate,
    ui_widgets::{Activate, Button},
};

use crate::{CYAN, GameState, ORANGE, screens::game::score::Score};

pub struct EndgamePlugin;

impl Plugin for EndgamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EndGame), end_game_ui)
            .add_systems(OnExit(GameState::EndGame), cleanup_end_game_ui);
    }
}

#[derive(Component, FromTemplate, Clone)]
struct EndGameUi;

fn end_game_ui(mut commands: Commands, score: Res<Score>) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        EndGameUi
        Children [
            @endgame_background(&score)
        ]
    });
}

fn endgame_background(score: &Score) -> impl Scene {
    bsn! {
        Node {
            width: percent(30),
            height: percent(40),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(30),
        }
        BackgroundColor(Color::hsl(223., 0.30, 0.09))
            BorderGradient::from(LinearGradient {
                angle: FRAC_2_PI,
                stops: vec![
                    ColorStop::new(CYAN, percent(0)),
                    ColorStop::new(CYAN, percent(50)),
                    ColorStop::new(ORANGE, percent(50)),
                    ColorStop::new(ORANGE, percent(100)),
                ],
                ..default()
            })
        Children [
            @game_over_text()
                --
                @divider()
                --
                @score_texts(score)
                --
                @retry_button()
        ]
    }
}

fn game_over_text() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(16),
        }
        Children [
            Text("GAME")
            TextFont {
                font_size: px(60),
                font: FontSourceTemplate::Handle(
                    "fonts/inter/static/Inter_28pt-Light.ttf"
                ),
            }
            TextColor(CYAN)
            --
            Text("OVER")
            TextFont {
                font_size: px(60),
                font: FontSourceTemplate::Handle(
                    "fonts/inter/static/Inter_28pt-Light.ttf"
                ),
            }
            TextColor(ORANGE)
        ]
    }
}

fn divider() -> impl Scene {
    bsn! {
        Node {
            width: percent(40),
            height: px(2),
        }
        BackgroundGradient::from(LinearGradient {
            angle: std::f32::consts::FRAC_PI_2,
            stops: vec![
                ColorStop::new(CYAN, percent(0)),
                ColorStop::new(CYAN, percent(50)),
                ColorStop::new(ORANGE, percent(50)),
                ColorStop::new(ORANGE, percent(100)),
            ],
            ..default()
        })
    }
}

fn score_texts(score: &Score) -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(7.5)
        }
        Children [
            @score_text()
            --
            @final_score_text(score)
        ]
    }
}

fn score_text() -> impl Scene {
    bsn! {
        Text("score")
        TextFont {
            font_size: px(27.5),
            font: FontSourceTemplate::Handle("fonts/inter/static/Inter_28pt-Light.ttf"),

        }
        TextColor(Color::hsl(225., 0.06, 0.52))
    }
}

fn final_score_text(final_score: &Score) -> impl Scene {
    let score = format!("{:04}", final_score.0);
    bsn! {
        Text(score)
        TextFont {
            font_size: px(45),
            font: FontSourceTemplate::Handle("fonts/inter/static/Inter_28pt-Light.ttf"),

        }
        TextColor(Color::WHITE)
    }
}

fn retry_button() -> impl Scene {
    bsn! {
        Button
        Node {
            width: percent(55),
            height: percent(17.5),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }

        BackgroundColor(Color::hsl(223., 0.30, 0.09))
            BorderGradient::from(LinearGradient {
                angle: -FRAC_2_PI,
                stops: vec![
                    ColorStop::new(CYAN, percent(0)),
                    ColorStop::new(CYAN, percent(50)),
                    ColorStop::new(ORANGE, percent(50)),
                    ColorStop::new(ORANGE, percent(100)),
                ],
                ..default()
            })
        on(|_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
            next_state.set(GameState::SplashScreen);
        })
        Children [
            Text("R E T R Y")
            TextFont {
                font_size: px(23),
                font: FontSourceTemplate::Handle(
                    "fonts/inter/static/Inter_28pt-Light.ttf"
                ),
            }
            TextColor(Color::WHITE)
        ]
    }
}

fn cleanup_end_game_ui(
    mut commands: Commands,
    end_game_ui_entity: Single<Entity, With<EndGameUi>>,
) {
    commands.entity(*end_game_ui_entity).despawn();
}
