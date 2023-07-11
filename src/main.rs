use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Pad;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, query_pads)
        // .add_systems(Update)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(Ball);
    commands.spawn(Pad);
    commands.spawn(Pad);
    
}

fn query_pads(query: Query<&Pad>) {
    for pad in &query {
        println!("pad");
    }

}



fn print_fps(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        println!("FPS: {:?}", fps.value().unwrap_or(f64::NAN));
    }
}

