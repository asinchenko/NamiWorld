mod menu;
mod pug;

use bevy::render::view::Layer;
use bevy::{core::FixedTimestep, ecs::schedule::SystemSet, prelude::*};

use menu::*;
use menu::*;
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

pub const JUMP_IMPULSE: f32 = 500.0;
pub const PLAYER_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_system(window_resize_system)
        .add_plugin(PugPlugin)
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .run();
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let windowresolution = WindowRes::full_hd();
    window.set_resolution(windowresolution.x, windowresolution.y);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Pause,
}
