use crate::pug::Jumper;
use crate::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn return_rigid_body(
    position_x: f32,
    position_y: f32,
    rigidbody_type: &str,
    gravity: f32,
) -> RigidBodyBundle {
    let rigidbody_type_fmt = &rigidbody_type.to_lowercase();
    let static_res: &'static str = "static";
    let mut result = RigidBodyTypeComponent(RigidBodyType::Dynamic);
    if rigidbody_type_fmt == static_res {
        result = RigidBodyTypeComponent(RigidBodyType::Static);
    }
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsComponent(RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()
        }),
        position: RigidBodyPositionComponent(RigidBodyPosition {
            position: Vec2::new(position_x, position_y).into(),
            ..Default::default()
        }),
        body_type: result,
        forces: RigidBodyForcesComponent(RigidBodyForces {
            gravity_scale: gravity,
            ..Default::default()
        }),
        activation: RigidBodyActivationComponent(RigidBodyActivation::active()),
        //collision detection
        ccd: RigidBodyCcdComponent(RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }),
        ..Default::default()
    };
    return rigid_body;
}
pub fn return_collider(cuboid_x: f32, cuboid_y: f32, cuboid_z: f32) -> ColliderBundle {
    let collider = ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::round_cuboid(cuboid_x, cuboid_y, cuboid_z)),
        mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(200.0).into()),
        flags: ColliderFlagsComponent(ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }),
        ..Default::default()
    };
    return collider;
}
pub fn rigid_body_position_sync() -> RigidBodyPositionSync {
    let discrete = RigidBodyPositionSync::Discrete;
    discrete
}

pub fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut RigidBodyVelocityComponent), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) && !jumper.is_jumping {
            velocity.linvel = Vec2::new(0., jumper.jump_impulse).into();
            jumper.is_jumping = true
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(entity: Entity, jumper: &mut Jumper, event: &ContactEvent) {
    if let ContactEvent::Started(h1, h2) = event {
        if h1.entity() == entity || h2.entity() == entity {
            jumper.is_jumping = false
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut RigidBodyVelocityComponent, &mut Sprite)>,
) {
    for (player, mut velocity, mut sprite) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
            sprite.flip_x = false;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
            sprite.flip_x = true;
        }
    }
}
pub fn death_by_height(
    mut commands: Commands,
    players: Query<(Entity, &RigidBodyPositionComponent), With<Player>>,
) {
    for (entity, position) in players.iter() {
        if position.position.translation.y < -200. {
            commands.entity(entity).despawn();
        }
    }
}