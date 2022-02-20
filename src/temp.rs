use bevy::prelude::*;

const PLAYER_SPRITE: &str = "pug.png";
const TIME_STEP: f32 = 1. /60.;
pub struct Materials{
    player_materials: Handle<ColorMaterial>,
}

struct WinSize {
    w: f32,
    h: f32
}
#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerSpeed {
    speed: f32,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor{
        title: "Puggy Wuggy".to_string(),
        width: 600.0,
        height: 676.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    //.add_startup_stage("game_setup_actors", SystemStage::single(player_spawn.system()))
    .add_startup_system(player_spawn.system())
    .add_system(player_movement.system())
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    let mut window = windows.get_primary_mut().unwrap();
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window
    window.set_position(IVec2::new(3870,4830));

    commands.insert_resource(Materials{
        player_materials: asset_server.load(PLAYER_SPRITE),
    });
    commands.insert_resource(WinSize{
        w: window.width(),
        h:window.height(),
    })
}


fn player_spawn(mut commands: Commands,
_materials: Res<Materials>,
win_size: Res<WinSize>,
asset_server: Res<AssetServer>,
){
    //spawn a sprite
    let bottom = -win_size.h / 2.0;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0. ,bottom + 200./2. +5.,10.),
            scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player) 
    .insert(PlayerSpeed{speed: 500.0});
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
) {
    for (speed, mut transform, _) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * speed.speed;
        // bound the paddle
        translation.x = translation.x.min(380.0).max(-380.0);
    }
}
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
// ){
//     if let Ok((speed, mut transform, _)) = query.single_mut(){
//         let dir = if keyboard_input.pressed(KeyCode::Left) {
//             -1.
//         }else if keyboard_input.pressed(KeyCode::Right) {
//             1.
//         }else {
//             0.
//         };
//     transform.translation.x += dir*speed.speed* TIME_STEP;}
// }