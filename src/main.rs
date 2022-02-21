mod pug;

use bevy::render::view::Layer;
use bevy::{
    core::FixedTimestep, ecs::schedule::SystemSet, prelude::*, render::camera::CameraPlugin,
};

use pug::*;
use rand::Rng;

struct WindowRes {
    x: f32,
    y: f32,
}
impl WindowRes {
    fn full_hd() -> WindowRes {
        WindowRes { x: 1920., y: 1080. }
    }
    fn hd() -> WindowRes {
        WindowRes { x: 1280., y: 720. }
    }
}

pub const JUMP_IMPULSE: f32 = 200.0;
pub const PLAYER_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_system(window_resize_system.system())
        .add_plugin(PugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let windowresolution = WindowRes::full_hd();
    window.set_resolution(windowresolution.x, windowresolution.y);
}
