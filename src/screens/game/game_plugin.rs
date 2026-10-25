use crate::{GameState, screens::game::enemy_systems::*};
use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
struct GameSet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_systems(OnEnter(GameState::Game), spawn_ui)
            .add_systems(
                Update,
                (maintain_enemies, move_enemies, despawn_enemies).in_set(GameSet),
            );
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
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
