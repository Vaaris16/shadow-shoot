use bevy::prelude::*;
use rand::RngExt;

const ENEMY_SPRITE_SHADOW_SQUARE: &str = "enemies/enemy_2_square.png";
const ENEMY_SPRITE_SHADOW_TRIANGLE: &str = "enemies/enemy_2_triangle.png";
const ENEMY_SPRITE_PLAYER_SQUARE: &str = "enemies/enemy_1_square.png";
const ENEMY_SPRITE_PLAYER_TRIANGLE: &str = "enemies/enemy_1_triangle.png";

#[derive(Copy, Clone)]
pub enum EnemyType {
    Player,
    Shadow,
}

#[derive(Component)]
pub struct Enemy {
    pub velocity: Vec2,
    enemy_type: EnemyType,
    pub pos: Vec2,
    pub enemy_sprite: String,
}

impl Enemy {
    pub fn new(e_t: EnemyType) -> Self {
        Self {
            velocity: Enemy::rand_vel(),
            enemy_type: e_t,
            pos: Vec2::ZERO,
            enemy_sprite: String::from(Enemy::enemy_sprite_path(&e_t)),
        }
    }

    fn rand_vel() -> Vec2 {
        let mut rng = rand::rng();
        let y_v = rng.random_range(-5.0..-3.);
        Vec2 { x: 0., y: y_v }
    }

    fn enemy_sprite_path(enemy_type: &EnemyType) -> &str {
        let mut rng = rand::rng();
        match enemy_type {
            EnemyType::Player => match rng.random_range(0..2) {
                0 => ENEMY_SPRITE_PLAYER_SQUARE,
                1 => ENEMY_SPRITE_PLAYER_TRIANGLE,
                _ => unreachable!(),
            },
            EnemyType::Shadow => match rng.random_range(0..2) {
                0 => ENEMY_SPRITE_SHADOW_SQUARE,
                1 => ENEMY_SPRITE_SHADOW_TRIANGLE,
                _ => unreachable!(),
            },
        }
    }

    pub fn rand_pos(&mut self, window: &Window) {
        let mut rng = rand::rng();
        let x = match self.enemy_type {
            EnemyType::Player => rng.random_range(-window.width() / 2.0..-10.0),
            EnemyType::Shadow => rng.random_range(10.0..window.width() / 2.0),
        };
        self.pos = Vec2 {
            x,
            y: window.height() / 2. + 40.,
        };
    }
}
