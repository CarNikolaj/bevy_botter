use bevy::prelude::*;
// import the primitive from the prelude (or bevy::math::primitives::Sphere)


pub fn lighting(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75,
        affects_lightmapped_meshes: true,
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 50_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(20.0, 40.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}



pub fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Mesh: simple sphere with radius 3.0
    let mesh = meshes.add(Mesh::from(Sphere { radius: 3.0, ..default() }));

    // Material: yellow-ish
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.2),
        // optional "glow":
        // emissive: Color::srgb(1.0, 0.9, 0.2),
        ..default()
    });        


    }