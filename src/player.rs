use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) && transform.translation.y < 250.0 {
        transform.translation.y += 300.0 * time.delta_seconds() * TILE_SIZE;
    }
    if keyboard.pressed(KeyCode::S) && transform.translation.y > -250.0 {
        transform.translation.y -= 300.0 * time.delta_seconds() * TILE_SIZE;
    }
}

fn spawn_player(mut commands: Commands) {
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
            transform: Transform::from_translation(Vec3::new(-350.0, 0.0, 0.0)),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player {});
}
