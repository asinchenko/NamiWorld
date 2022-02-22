mod camera;
mod myphysics;

use crate::pug::camera::new_camera_2d;
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
struct Direction {
    is_left: bool,
    is_right: bool,
}

#[derive(Component)]
pub struct Jumper {
    jump_impulse: f32,
    is_jumping: bool,
}

pub struct PugPlugin;

impl Plugin for PugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_system_set(
                SystemSet::on_enter(crate::AppState::InGame)
                    .with_system(sprite_spawn)
                    .with_system(spawn_floor),
            )
            .add_system_set(
                SystemSet::on_update(crate::AppState::InGame)
                    .with_system(myphysics::player_jumps)
                    .with_system(myphysics::jump_reset)
                    .with_system(myphysics::player_movement),
            )
            .add_system_set(SystemSet::on_enter(crate::AppState::MainMenu).with_system(cleanup))
            .add_system_set(SystemSet::on_exit(crate::AppState::MainMenu).with_system(cleanup));
    }
}
//spawn Puggy
fn sprite_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 300.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("background.png"),
        ..Default::default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            texture: asset_server.load("Pug.png"),
            sprite: Sprite {
                flip_y: false,
                flip_x: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(myphysics::return_rigid_body(0.0, 500.0, "dynamic", 200.0))
        .insert_bundle(myphysics::return_collider(0.5, 0.5, 0.1))
        .insert(myphysics::rigid_body_position_sync())
        .insert(Player {
            speed: crate::PLAYER_SPEED,
        })
        .insert(Jumper {
            jump_impulse: crate::JUMP_IMPULSE,
            is_jumping: false,
        })
        .with_children(|parent| {
            parent.spawn_bundle(new_camera_2d());
        });
}

pub fn spawn_floor(mut commands: Commands) {
    add_tile(&mut commands, 0.);
    for x in 0..2000 {
        if x % 200 == 0 {
            add_tile(&mut commands, x as f32)
        }
    }
}

fn add_tile(commands: &mut Commands, x: f32) {
    let width = 200.;
    let height = 50.;
    let size = Vec2::new(width, height);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(myphysics::return_rigid_body(x, 0. - 100., "Static", 0.))
        .insert_bundle(myphysics::return_collider(110., 100., 0.1))
        .insert(myphysics::rigid_body_position_sync());
}

pub fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// fn spawn_floor(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
// let width = 1920.;
// let height = 50.;
// let size = Vec2::new(1920., 50.0);
// commands
// .spawn_bundle(SpriteBundle {
//     sprite: Sprite {
//         color: Color::WHITE,
//         custom_size: Some(size),
//         ..Default::default()
//     },
//     transform: Transform::from_translation(Vec3::new(0.0, -100.0, 1.0)),
//     ..Default::default()
// })
//     .insert_bundle(myphysics::return_rigid_body(0., 0., "Static", 1.))
//     .insert_bundle(myphysics::return_collider(width/2., height/2.))
//     .insert(myphysics::rigid_body_position_sync());
// }
