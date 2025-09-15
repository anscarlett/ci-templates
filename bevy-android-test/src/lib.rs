use bevy::prelude::*;

mod shapes_demo;

// Re-export for non-Android platforms
pub use shapes_demo::main;

// JNI entry point for Android
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use bevy::winit::WinitPlugin;
    
    // Initialize logging for Android
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));
    
    // Insert the AndroidApp resource and run the Bevy app
    App::new()
        .insert_resource(app)
        .add_plugins((
            DefaultPlugins.set(WinitPlugin::default()).set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            bevy::pbr::wireframe::WireframePlugin::default(),
        ))
        .add_systems(Startup, shapes_demo::setup)
        .add_systems(
            Update,
            (
                shapes_demo::rotate,
                #[cfg(not(target_arch = "wasm32"))]
                shapes_demo::toggle_wireframe,
            ),
        )
        .run();
}