use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
  


pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // Load the first scene from the GLB
    let sponza_scene: Handle<Scene> =
        assets.load(GltfAssetLabel::Scene(0).from_asset(""));

    commands.spawn((
        SceneRoot(sponza_scene),
        Transform::default(),
        Visibility::default(),
        Name::new("Sponza Scene")
    ));

  

    
}


