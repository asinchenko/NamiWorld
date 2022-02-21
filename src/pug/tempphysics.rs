mod camera;
mod myphysics;

use crate::pug::camera::new_camera_2d;
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
struct Jumper {
    jump_impulse: f32,
    is_jumping: bool,
}

pub struct PugPlugin;

impl Plugin for PugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(sprite_spawn)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_startup_system(spawn_floor)
        ;
    }
}

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
        .insert_bundle(myphysics::return_rigid_body(0.5, 0.5, "dynamic"))
        .insert_bundle(myphysics::return_collider(0.5, 0.5))
        .insert(Player {speed: crate::PLAYER_SPEED})
        .with_children(|parent| {
          parent.spawn_bundle(new_camera_2d());
        });
}

fn spawn_floor(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
  let width = 10.;
  let height = 1.;
  let size = Vec2::new(1920., 50.0);
  commands
  .spawn_bundle(SpriteBundle {
      sprite: Sprite {
          color: Color::WHITE,
          custom_size: Some(size),
          ..Default::default()
      },
      transform: Transform::from_translation(Vec3::new(0.0, -100.0, 1.0)),
      ..Default::default()
  })
      .insert_bundle(myphysics::return_rigid_body(width / 2., height / 2., "Static"))
      .insert_bundle(myphysics::return_collider(50., 50.));
}