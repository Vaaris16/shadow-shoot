use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score(pub i64);

#[derive(Component, Clone, FromTemplate)]
pub struct ScoreText;

#[derive(Resource)]
pub struct ScoreStepTimer(pub Timer);

pub fn add_score(
    time: Res<Time>,
    mut score_timer: ResMut<ScoreStepTimer>,
    mut score: ResMut<Score>,
) {
    score_timer.0.tick(time.delta());
    if score_timer.0.just_finished() {
        score.0 += 1;
    }
}

pub fn set_score(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    let score_string = format!("{:04}", score.0);
    score_text.0 = score_string;
}

const SCORE_TEXT_COLOR: Color = Color::hsl(206., 0.65, 0.67);
pub fn spawn_score() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            right: px(20),
        }
        Children [
            Text("Score")
            TextFont {
                font_size: px(65)
            }
            TextColor(SCORE_TEXT_COLOR)
            ScoreText
        ]
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}
