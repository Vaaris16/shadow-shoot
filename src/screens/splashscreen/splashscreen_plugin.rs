use std::ops::BitAndAssign;

use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::{GameState, screens::splashscreen};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splash_screen)
            .add_systems(OnExit(GameState::SplashScreen), cleanup_splashscreen);
    }
}

#[derive(Component, Clone, FromTemplate)]
struct SplashScreen;

fn spawn_splash_screen(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        SplashScreen
        Children [
            @spawn_splash_title()
            --
            @start_button()
        ]
    });
}

fn spawn_splash_title() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(15),
        }
        Children [
            Text("The")
            TextFont {
                font_size: px(40),
            }
            TextColor(Color::WHITE)
            --
            Text("Other")
            TextFont {
                font_size: px(40),
            }
            TextColor(Color::WHITE)
            --
            Text("Side")
            TextFont {
                font_size: px(40),
            }
            TextColor(Color::WHITE)
        ]
    }
}

fn start_button() -> impl Scene {
    bsn! {
        Button
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        on(start_game)
        Children [
            Text("Start Game")
            TextFont {
                font_size: px(24),
            }
            TextColor(Color::WHITE)
        ]
    }
}

fn start_game(_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Game);
}

fn cleanup_splashscreen(mut commands: Commands, splash_screen: Single<Entity, With<SplashScreen>>) {
    commands.entity(*splash_screen).despawn();
}
