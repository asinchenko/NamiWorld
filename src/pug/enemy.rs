use rand::thread_rng;
use rand::Rng;
use super::myphysics;
use bevy::prelude::*;
use myphysics::*;
use bevy_rapier2d::prelude::*;


#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Monster;


pub fn insert_monster_at(commands: &mut Commands, x: f32, y: f32) {
    let rigid_body = RigidBodyBundle {
        position: RigidBodyPositionComponent(RigidBodyPosition {
            position: Vec3::new(x as f32, y as f32, 1.).into(),
            ..Default::default()
        }),
        mass_properties: RigidBodyMassPropsComponent(RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()}),
            activation: RigidBodyActivationComponent(RigidBodyActivation::active()),
        forces: RigidBodyForcesComponent(RigidBodyForces {
            gravity_scale: 200.,
            ..Default::default()
        }),
        body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::round_cuboid(30.35, 30.35, 0.1)),
        //mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(200.0).into()),
        flags: ColliderFlagsComponent(ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }),
        ..Default::default()
    };

    let sprite = SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        sprite:  Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            color: Color::RED,
            ..Default::default()},
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Enemy)
        .insert(Monster);
}

fn should_add_enemy(x: usize) -> bool {
    if x <= 5 {
        return false;
    }
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..100);
    match random_number {
        0..=90 => false,
        _ => true,
    }
}