mod camera;
mod myphysics;

use crate::pug::myphysics::death_by_height;
use crate::pug::camera::new_camera_2d;
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Floor{
  floor_entity: Entity,
}

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

#[derive(Component)]
pub struct PlayerData {
  player_entity: Entity,
  camera_entity: Entity,
}

pub struct PugPlugin;

impl Plugin for PugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(camera_follow_player.label("Camera system"))
            .add_system_set(
                SystemSet::on_enter(crate::AppState::InGame)
                    .with_system(sprite_spawn)
                    .with_system(spawn_floor),
            )
            .add_system_set(
                SystemSet::on_update(crate::AppState::InGame)
                    .with_system(check_delete)
                    .with_system(myphysics::player_jumps.label("Player Jumps System"))
                    .with_system(myphysics::jump_reset.label("Jump Reset System"))
                    .with_system(myphysics::player_movement.label("Player Movement System"))
                    .with_system(death_by_height.label("Death system"))
                    ,
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup).with_system(cleanup_player))
            ;
    }
}
//spawn Puggy
fn sprite_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(SpriteBundle {
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 300.0, 0.0),
    //         ..Default::default()
    //     },
    //     texture: asset_server.load("background.png"),
    //     ..Default::default()
    // });
    let player_entity = commands
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
        }).id()
        ;
      let camera_entity = commands.spawn_bundle(new_camera_2d()).id();
        commands.insert_resource(PlayerData {
        player_entity,
        camera_entity,
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
  let floor_entity = commands
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
      .insert(myphysics::rigid_body_position_sync())
      .id();
  commands.insert_resource(Floor {
    floor_entity,
});
}



fn camera_follow_player(
  mut cameras: Query<&mut Transform, With<Camera>>,
  players: Query<&RigidBodyPositionComponent, With<Player>>,
) {
  for player in players.iter() {
      for mut camera in cameras.iter_mut() {
          camera.translation.x = player.position.translation.x;
          camera.translation.y = player.position.translation.y;
      }
  }
}

fn cleanup_player(mut commands: Commands, player_data: Res<PlayerData>, query: Query<Entity>) {
  commands
      .entity(player_data.player_entity)
      .despawn_recursive();
  commands
      .entity(player_data.camera_entity)
      .despawn_recursive();
}

fn cleanup(mut commands: Commands, mut query: Query<Entity>) {
  for entity in query.iter() {
      commands.entity(entity).despawn_recursive();
  }
}

fn query_entities(mut commands: Commands,query: Query<Entity>){
  for entity in query.iter() {
    //println!{"{:?}", entity};
  }
  println!{"STOPPED"};
}

fn check_delete(
  keyboard_input: Res<Input<KeyCode>>,
  mut commands: Commands, mut player_data: Res<PlayerData>, query: Query<Entity>
) {
   if keyboard_input.pressed(KeyCode::Down) {
  commands
      .entity(player_data.player_entity)
      .despawn_recursive();
   commands
       .entity(player_data.camera_entity)
       .despawn_recursive();
    }
  }