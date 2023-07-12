use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*, sprite::MaterialMesh2dBundle, window::{PresentMode, WindowTheme},
};

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct LeftPad;

#[derive(Component)]
struct RightPad;

#[derive(Resource)]
struct BallVelocity(Vec2);


const BALL_SPEED: f32 = 300.0;
const BALL_SIZE: f32 = 32.;

const GAME_FRAME_WIDTH: f32 = 700.;
const GAME_FRAME_HEIGHT: f32 = 450.;
const GAME_FRAME_THICKNESS: f32 = 5.;
const GAME_FRAME_POSITION: Vec2 = Vec2::new(0., -25.);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ping Pong Game in Rust".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()

        }), FrameTimeDiagnosticsPlugin))
        .insert_resource(BallVelocity(Vec2::new(0.0, 1.0)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, startup)
        .add_systems(Update, (print_fps, ball_movement, ball_collision))
        // .add_systems(Update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((Ball, MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
        transform: Transform::default().with_translation(Vec3::new(0., 0., 3.)).with_scale(Vec3::splat(BALL_SIZE)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    }));


    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_translation(Vec3::new(GAME_FRAME_POSITION.x, GAME_FRAME_POSITION.y, 0.)).with_scale(Vec3::new(GAME_FRAME_WIDTH, GAME_FRAME_HEIGHT, 0.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_translation(Vec3::new(GAME_FRAME_POSITION.x, GAME_FRAME_POSITION.y, 1.)).with_scale(Vec3::new(GAME_FRAME_WIDTH - 2. * GAME_FRAME_THICKNESS, GAME_FRAME_HEIGHT - 2. * GAME_FRAME_THICKNESS, 0.)),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..default()
    });


    commands.spawn(LeftPad);
    commands.spawn(RightPad);
    
}

fn ball_movement(
    mut query: Query<(&mut Ball, &mut Transform)>,
    ball_vel: Res<BallVelocity>,
    time: Res<Time>
) {
    let (_ball, mut transform) = query.iter_mut().next().unwrap();
    transform.translation += Vec3::new(ball_vel.0.x, ball_vel.0.y, 0.) * BALL_SPEED * time.delta_seconds();
}


fn ball_collision(
    mut query: Query<(&mut Ball, &mut Transform)>,
    mut ball_vel: ResMut<BallVelocity>
) {
    let (_ball, transform) = query.iter_mut().next().unwrap();
    
    
    if transform.translation.y >= GAME_FRAME_POSITION.y + GAME_FRAME_HEIGHT/2. - GAME_FRAME_THICKNESS/2. - BALL_SIZE/2. || transform.translation.y <= GAME_FRAME_POSITION.y - GAME_FRAME_HEIGHT/2. + GAME_FRAME_THICKNESS/2. + BALL_SIZE/2. {
        ball_vel.0.y *= -1.
    }
    
}



fn print_fps(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        println!("FPS: {:?}", fps.value().unwrap_or(f64::NAN));
    }
}

