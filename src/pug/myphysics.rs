use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn return_rigid_body(position_x:f32, position_y:f32, rigidbody_type:&str, gravity:f32) -> RigidBodyBundle {
    let rigidbody_type_fmt = &rigidbody_type.to_lowercase();
    let static_res: &'static str = "static";
    let mut result = RigidBodyTypeComponent(RigidBodyType::Dynamic);
    if rigidbody_type_fmt == static_res {
        result = RigidBodyTypeComponent(RigidBodyType::Static);
        
    }
    let rigid_body = RigidBodyBundle {
        mass_properties: RigidBodyMassPropsComponent(RigidBodyMassProps{ flags: RigidBodyMassPropsFlags::ROTATION_LOCKED, ..Default::default() }),
        position:  RigidBodyPositionComponent(RigidBodyPosition{position: Vec2::new(position_x, position_y).into(), ..Default::default()}),
        body_type: result,
        forces:  RigidBodyForcesComponent(RigidBodyForces{gravity_scale: gravity, ..Default::default()}),
        activation: RigidBodyActivationComponent(RigidBodyActivation::cannot_sleep()),
        //collision detection
        ccd:  RigidBodyCcdComponent(RigidBodyCcd{ccd_enabled: true, ..Default::default()}),
        ..Default::default()
    };
    return rigid_body
}
pub fn return_collider(cuboid_x:f32,cuboid_y:f32) -> ColliderBundle {
    let collider = ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::cuboid(cuboid_x,cuboid_y)),
        mass_properties: ColliderMassPropsComponent(ColliderMassProps::Density(200.0).into()),
        flags: ColliderFlagsComponent(ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }),
        ..Default::default()
    };
    return collider
}

//never used for now
pub fn rigid_body_position_sync() -> RigidBodyPositionSync {
    let discrete = RigidBodyPositionSync::Discrete;
    discrete
}