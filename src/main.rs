use bevy::prelude::*;

use crate::player::PlayerPlugin;

mod player;

pub const TILE_SIZE: f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 600.0,
                title: String::from("Pong"),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
