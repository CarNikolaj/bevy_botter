use bevy::prelude::*;
use bevy::window::{
    Window, WindowPlugin, WindowMode, WindowResolution, WindowPosition,
    MonitorSelection, PresentMode,
};

pub fn configured_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Mein Spiel".into(),
            resolution: WindowResolution::new(1920.0, 1080.0),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            present_mode: PresentMode::AutoVsync,
            //resizable: true,
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }
}