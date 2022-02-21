use bevy::{
    math::Vec3,
    prelude::OrthographicCameraBundle,
    render::camera::{DepthCalculation, OrthographicProjection, ScalingMode},
};

pub fn new_camera_2d() -> OrthographicCameraBundle {
    let far = 1000.0;
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far,
        top: 1000.0,
        depth_calculation: DepthCalculation::ZDifference,
        //scaling_mode: ScalingMode::FixedHorizontal,
        ..Default::default()
    };
    camera.transform.translation = Vec3::new(0., 300., 999.);
    return camera;
}
