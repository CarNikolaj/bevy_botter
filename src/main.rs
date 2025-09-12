mod window;
mod camera;
mod scene;
mod spectator;
mod light;
mod diagnostics;
mod window_settings;

//Own
use spectator::{
    spectator_look, 
    spectator_move
};
use window::{
    apply_grab, 
    focus_events, 
    toggle_grab
};
use camera::{ 
    spawn_camera_spectator, 
    disable_imported_cameras
};
use scene::{
    setup
};

use light::{
    lighting,
    spawn_sun
};

use diagnostics::{
    performance
};

use window_settings::{
    configured_window_plugin
};


//Bevy
use bevy::prelude::*;
use bevy::input::common_conditions::input_just_released;
use bevy::asset::AssetPlugin;
use iyes_perf_ui::prelude::*;




fn main() {
    


    let asset_plugin = AssetPlugin {
        watch_for_changes_override: Some(true),     // Hot-Reload
        ..default()
    };

   
    let mut app = App::new();

    //Window settings
    
    //Plugins
    app.add_plugins(DefaultPlugins
        .set(asset_plugin)
        .set(configured_window_plugin())
    );
    app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
    app.add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(PerfUiPlugin);

    // Ressources
    app.insert_resource(Time::<Fixed>::from_hz(30.));

    // Systems
    app.add_systems(
        Startup,
        (setup, spawn_camera_spectator, lighting, performance, spawn_sun),
    );

    app.add_systems(
        Update,
        (
            spectator_look,
            spectator_move.after(spectator_look),
            focus_events,
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
        ),
    );

    // z. B. HDR global ausschalten (oder nur bei deiner aktiven Kamera)
    app.add_systems(Startup, |mut q: Query<&mut Camera, With<Camera3d>>| {
        for mut cam in &mut q { cam.hdr = false; }
    });

    app.add_systems(Update, disable_imported_cameras);

    app.add_observer(apply_grab);

    app.run();
}




