use bevy::prelude::*;


pub struct Background{
    background: Entity,
  }

fn background(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32) {
    let background = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, 300.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("background.png"),
        ..Default::default()
    }).id();
    commands.insert_resource(Background {
        background,
    });
}

pub fn spawn_background(mut commands: Commands,asset_server: Res<AssetServer>) {
    background(&mut commands, &asset_server, 0. as f32);
    background(&mut commands, &asset_server, 1920. as f32);
    background(&mut commands, &asset_server, -1920. as f32);

  }