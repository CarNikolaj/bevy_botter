use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::{Window, PrimaryWindow};





const SPECTATOR_SPEED: f32 = 5.0;

#[derive(Component)]
pub struct Spectator;


pub fn spectator_look(
    mut spectator: Single<&mut Transform, With<Spectator>>,
    mouse_movement: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }

    let sensitivity = 100. / window.width().min(window.height());

    let (mut yaw, mut pitch, _) = spectator.rotation.to_euler(EulerRot::YXZ);

    pitch -= mouse_movement.delta.y * time.delta_secs() * sensitivity;

    yaw -= mouse_movement.delta.x * time.delta_secs() * sensitivity;

    pitch = pitch.clamp(-1.57, 1.57);

    spectator.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
}


pub fn spectator_move(
    mut spectator: Single<&mut Transform, With<Spectator>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    
) {  
    // input
    let mut d = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA)        { d.x -= 1.0; }
    if input.pressed(KeyCode::KeyD)        { d.x += 1.0; }
    if input.pressed(KeyCode::KeyW)        { d.z += 1.0; } 
    if input.pressed(KeyCode::KeyS)        { d.z -= 1.0; }
    if input.pressed(KeyCode::Space)       { d.y += 1.0; } 
    if input.pressed(KeyCode::ShiftLeft)   { d.y -= 1.0; } 

    // yaw
    let (yaw, _pitch, _roll) = spectator.rotation.to_euler(EulerRot::YXZ);
    let yaw_only = Quat::from_rotation_y(yaw);

    // Flache (XZ) Richtungsbasen aus NUR Yaw berechnen
    // -Z ist "vorwärts", +X ist "rechts"
    let forward_flat = yaw_only * -Vec3::Z;
    let right_flat   = yaw_only *  Vec3::X;

    // Horizontalbewegung (WASD) in der Horizontalen,
    // Vertikal separat addieren 
    let horiz = forward_flat * d.z + right_flat * d.x;
    let vert  = Vec3::Y * d.y;

    let dir = (horiz + vert).normalize_or_zero();

    spectator.translation += dir * time.delta_secs() * SPECTATOR_SPEED; 
}

