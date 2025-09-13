use crate::spectator::Spectator;
use bevy::prelude::*;
use bevy::core_pipeline::Skybox;


pub fn spawn_camera_spectator(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        // optional, aber oft hübscher:
        bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
        Spectator,
        Transform::from_xyz(-15.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Skybox {
            image: assets.load("sky/skybox.ktx2"),
            brightness: 1000.0,
            ..Default::default()
        },
    ));
}


pub fn disable_imported_cameras(
    mut q: Query<&mut Camera, (Added<Camera>, Without<Spectator>)>,
) {
    for mut cam in &mut q {
        cam.is_active = false;
        // optional: verhindert erneutes „Leer“-Clearen, falls du sie mal aktivierst
        cam.clear_color = bevy::render::camera::ClearColorConfig::None;
    }
}