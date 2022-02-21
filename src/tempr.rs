use bevy::{core::FixedTimestep, ecs::schedule::SystemSet, prelude::*, render::camera::CameraPlugin,};
//use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};
use bevy::render::view::Layer;
use heron::prelude::*;
use rand::Rng;
//use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};
//#[derive(Inspectable, Component)]
//struct InspectableType;
mod camera;
pub use camera::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ReflectedType;

const JUMP_IMPULSE: f32 = 200.0;
const PLAYER_SPEED: f32 = 200.0;

struct WindowRes {
  x: f32, 
  y: f32,
}
impl WindowRes {
  fn full_hd() -> WindowRes {
    WindowRes {x: 1920., y: 1080.}}
  fn hd() -> WindowRes {
    WindowRes {x:1280., y:720.}  }}
   
#[derive(Component)]
struct Direction {
  is_left: bool,
  is_right: bool,
}

#[derive(Component)]
struct Jumper {
  jump_impulse: f32,
  is_jumping: bool
}
#[derive(Component)]
pub struct Player{
  speed: f32,
}

fn main() {
  App::new()
    .add_system(window_resize_system.system())
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
    .insert_resource(Gravity::from(Vec3::new(0.0, -600.0, 0.0))) // Optionally define gravity
    //.add_plugin(WorldInspectorPlugin::new())
    //.register_inspectable::<InspectableType>() // tells bevy-inspector-egui how to display the struct in the world inspector
    //.register_type::<ReflectedType>() // registers the type in the `bevy_reflect` machinery, so that even without implementing `Inspectable` we can display the struct fields
    .add_startup_system(sprite_spawn)
    .add_plugin(MyPhysicsPlugin)
    .add_system(player_jumps)
    .add_system(detect_collisions_for_jumps)
    .add_system(player_movement)
    .run();
}

fn window_resize_system(mut windows: ResMut<Windows>) {
  let window = windows.get_primary_mut().unwrap();
  //println!("Window size was: {},{}", window.width(), window.height());
  let windowresolution = WindowRes::full_hd();
  window.set_resolution(windowresolution.x, windowresolution.y);
}
fn sprite_spawn(
    mut commands: Commands, asset_server: Res<AssetServer>) 
    {
    commands.spawn_bundle(SpriteBundle {
      texture: asset_server.load("background.png"),
      ..Default::default()
  });
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
          translation: Vec3::new(0.0, 0.0, 1.0),
          ..Default::default()},
        texture: asset_server.load("Pug.png"),
        sprite: Sprite {
          flip_y: false,
          flip_x: false,
          ..Default::default()},
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Sphere { radius: 10.0 })
    .insert(Velocity::from_linear(Vec3::X * 2.0))
    .insert(Acceleration::from_linear(Vec3::X * 1.0))
    .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
    .insert(RotationConstraints::lock())
    .insert(Jumper { jump_impulse: JUMP_IMPULSE, is_jumping: false })
    .insert(Player {speed: PLAYER_SPEED})
    .with_children(|parent| {
      parent.spawn_bundle(new_camera_2d());
    })
    .insert(Direction{ is_left: true, is_right:false})
    ;

    let size = Vec2::new(1920., 50.0);
    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
      ;
  }
  fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity, &mut Acceleration), With<main::Player>>
  ){
    for (mut jumper, mut velocity, mut acceleration) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            for i in 0..jumper.jump_impulse as i32{
              velocity.linear = Vec3::new(0., i as f32, 0.).into()};
            jumper.is_jumping = true;
        }
        if keyboard_input.just_released(KeyCode::Up) {
          velocity.linear = Vec3::new(velocity.linear.x, velocity.linear.y, 0.).into();
        }
    }
  } 
  fn detect_collisions_for_jumps(
    mut events: EventReader<CollisionEvent>,
    mut quaery: Query<(&mut Jumper, &mut Player)>)
    {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
              for (mut jumper, mut speed)in quaery.iter_mut(){
                jumper.is_jumping = false;
                speed.speed = PLAYER_SPEED;
                println!("Entity {:?} and {:?} started to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
            }}
            CollisionEvent::Stopped(data1, data2) => {
                for (mut jumper, mut speed) in quaery.iter_mut(){
                  jumper.is_jumping = true;
                  speed.speed = PLAYER_SPEED + 50.;
                  println!("Entity {:?} and {:?} stopped to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
                }
            }
        }
    }
  }
  fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &mut Velocity, &mut Direction, &mut Sprite), With<Player>>
  ){
    for (mut player, mut velocity, mut direction, mut sprite) in players.iter_mut() {
        let mut direction_count = 0.0;
        if keyboard_input.pressed(KeyCode::Left){
            velocity.linear = Vec3::new(-player.speed, velocity.linear.y, 0.).into();
            if direction.is_right == true{
              sprite.flip_x = false;
              direction.is_right = false;
              direction.is_left  = true;
            }
            }
        if keyboard_input.pressed(KeyCode::Right){
          velocity.linear = Vec3::new(player.speed, velocity.linear.y, 0.).into();
          if direction.is_left == true{
            sprite.flip_x = true;
            direction.is_right = true;
            direction.is_left  = false;
  
        }
      }
    }
  }