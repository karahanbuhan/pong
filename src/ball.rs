use bevy::prelude::*;
use rand::Rng;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball).add_system(ball_movement);
    }
}

fn ball_movement(mut ball_query: Query<(&mut Ball, &mut Transform)>, time: Res<Time>) {
    let (mut ball, mut transform) = ball_query.single_mut();

    if transform.translation.y > 300.0 || transform.translation.y < -300.0 {
        ball.velocity.y *= -1.0;
    }

    if transform.translation.x > 400.0 || transform.translation.x < -400.0 {
        ball.velocity.x *= -1.0;
    }

    transform.translation.x += 500.0 * ball.velocity.x * time.delta_seconds() * TILE_SIZE;
    transform.translation.y += 500.0 * ball.velocity.y * time.delta_seconds() * TILE_SIZE;
}

fn spawn_ball(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(15.0 * TILE_SIZE, 15.0 * TILE_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Ball"))
        .insert(Ball {
            velocity: Vec2::from_angle(45.0 - rand::thread_rng().gen_range(0.0..90.0)),
        });
}
