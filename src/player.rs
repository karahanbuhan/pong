use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct Player(Location);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_players)
            .add_system(player_movement);
    }
}

pub enum Location {
    Left,
    Right,
}

impl Location {
    pub fn to_vec(&self) -> Vec3 {
        match *self {
            Location::Left => Vec3::new(-350.0, 0.0, 0.0),
            Location::Right => Vec3::new(350.0, 0.0, 0.0),
        }
    }
}

const MOVE_SPEED: f32 = 400.0;

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (Player(location), mut transform) in player_query.iter_mut() {
        let mut move_vertical = |mul: f32| {
            transform.translation.y += mul * time.delta_seconds() * MOVE_SPEED * TILE_SIZE;
        };

        match location {
            Location::Left => {
                if keyboard.pressed(KeyCode::W) {
                    move_vertical(1.0);
                }
                if keyboard.pressed(KeyCode::S) {
                    move_vertical(-1.0);
                }
            }
            Location::Right => {
                if keyboard.pressed(KeyCode::Up) {
                    move_vertical(1.0);
                }
                if keyboard.pressed(KeyCode::Down) {
                    move_vertical(-1.0);
                }
            }
        }

        transform.translation.y = transform.translation.y.clamp(-250.0, 250.0)
    }
}

fn spawn_players(mut commands: Commands) {
    spawn_player(&mut commands, Location::Left);
    spawn_player(&mut commands, Location::Right);
}

fn spawn_player(commands: &mut Commands, location: Location) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0 * TILE_SIZE, 100.0 * TILE_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(SpatialBundle {
            transform: Transform::from_translation(location.to_vec()),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Player(location));
}
