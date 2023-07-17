use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::keyboard,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowTheme, WindowResolution}, text::DEFAULT_FONT_HANDLE,
};
use rand::Rng;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct LeftPad;

#[derive(Component)]
struct RightPad;

#[derive(Resource)]
struct BallVelocity(Vec2);

#[derive(Resource)]
struct PadInfo {
    height: f32,
}

#[derive(Resource)]
struct Score {
    left_score: u32,
    right_score: u32
}


#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct LeftScoreText;

#[derive(Component)]
struct RightScoreText;

const BALL_SPEED: f32 = 450.0;
const BALL_SIZE: f32 = 32.;

const GAME_FRAME_WIDTH: f32 = 700.;
const GAME_FRAME_HEIGHT: f32 = 450.;
const GAME_FRAME_THICKNESS: f32 = 5.;
const GAME_FRAME_POSITION: Vec2 = Vec2::new(0., -25.);

const PAD_SPEED: f32 = 10.;
const PAD_WIDTH: f32 = 10.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Ping Pong Game in Rust".into(),
                    resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(BallVelocity(Vec2::new(1.0, 1.0)))
        .insert_resource(ClearColor(Color::rgb(0.03, 0.03, 0.03)))
        .insert_resource(PadInfo { height: 80. })
        .insert_resource(Score { left_score: 0, right_score: 0 })
        .add_systems(Startup, startup)
        .add_systems(Update, (print_fps, ball_movement, ball_collision, pad_movement, text_update, restart))
        // .add_systems(Update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pad_info: Res<PadInfo>,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>
) {
    let window = windows.single();
    
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 0., 3.))
                .with_scale(Vec3::splat(BALL_SIZE)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_translation(Vec3::new(GAME_FRAME_POSITION.x, GAME_FRAME_POSITION.y, 0.))
            .with_scale(Vec3::new(GAME_FRAME_WIDTH, GAME_FRAME_HEIGHT + 2. * GAME_FRAME_THICKNESS, 0.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default()
            .with_translation(Vec3::new(GAME_FRAME_POSITION.x, GAME_FRAME_POSITION.y, 1.))
            .with_scale(Vec3::new(GAME_FRAME_WIDTH, GAME_FRAME_HEIGHT, 0.)),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..default()
    });

    commands.spawn((
        LeftPad,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(-GAME_FRAME_WIDTH/2. + 20., 0., 3.))
                .with_scale(Vec3::new(PAD_WIDTH, pad_info.height, 0.)),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        },
    ));

    commands.spawn((
        RightPad,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(GAME_FRAME_WIDTH/2. - 20., 0., 3.))
                .with_scale(Vec3::new(PAD_WIDTH, pad_info.height, 0.)),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        },
    ));

    commands.spawn((
        TextBundle::from_section(
        "Fps: ",
        TextStyle {
            font: DEFAULT_FONT_HANDLE.typed(),
            font_size: 16.,
            color: Color::WHITE,
        },
    )
    .with_text_alignment(TextAlignment::Left)
    .with_style(Style {
        position_type: PositionType::Absolute,
        left: Val::Px(15.),
        bottom: Val::Px(15.),

        ..Default::default()

        }),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_section(
        "0",
        TextStyle { 
            font: asset_server.load("fonts/bit5x3.ttf"), 
            font_size: 64., 
            color: Color::WHITE, 
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        left: Val::Px(window.width()/2. + GAME_FRAME_POSITION.x - GAME_FRAME_WIDTH/4.),
        top: Val::Px(20.),
        ..Default::default()
        }),
        LeftScoreText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
        "0",
        TextStyle { 
            font: asset_server.load("fonts/bit5x3.ttf"),
            font_size: 64., 
            color: Color::WHITE, 
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        left: Val::Px(window.width()/2. + GAME_FRAME_POSITION.x + GAME_FRAME_WIDTH/4.),
        top: Val::Px(20.),
        ..Default::default()
        }),
        RightScoreText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
        ":",
        TextStyle { 
            font: asset_server.load("fonts/bit5x3.ttf"),
            font_size: 64., 
            color: Color::WHITE, 
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        right: Val::Px(400.),
        top: Val::Px(15.),
        ..Default::default()
        }),
        RightScoreText,
    ));



}

fn ball_movement(
    mut query: Query<(&mut Ball, &mut Transform)>,
    ball_vel: Res<BallVelocity>,
    time: Res<Time>,
) {
    let (_ball, mut transform) = query.single_mut();
    transform.translation += Vec3::new(ball_vel.0.x, ball_vel.0.y, 0.) * BALL_SPEED * time.delta_seconds();
}

fn ball_collision(
    mut ball_vel: ResMut<BallVelocity>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<LeftPad>, Without<RightPad>)>,
    mut left_pad_query: Query<&mut Transform, (With<LeftPad>, Without<RightPad>, Without<Ball>)>,
    mut right_pad_query: Query<&mut Transform, (With<RightPad>, Without<LeftPad>, Without<Ball>)>,
    pad_info: Res<PadInfo>
) {
    let ball_transform = ball_query.single_mut();
    let left_pad_transform = left_pad_query.single_mut();
    let right_pad_transform = right_pad_query.single_mut();

    if ball_transform.translation.y
        >= GAME_FRAME_POSITION.y + GAME_FRAME_HEIGHT / 2. - BALL_SIZE / 2.
    {
        if ball_vel.0.y > 0. {
            ball_vel.0.y *= -1.
        }
    } else if ball_transform.translation.y
        <= GAME_FRAME_POSITION.y - GAME_FRAME_HEIGHT / 2. + BALL_SIZE / 2.
    {
        if ball_vel.0.y < 0. {
            ball_vel.0.y *= -1.
        }
    }

    // if ball_transform.translation.x
    //     >= GAME_FRAME_POSITION.x + GAME_FRAME_WIDTH / 2. - BALL_SIZE / 2.
    // {
    //     if ball_vel.0.x > 0. {
    //         ball_vel.0.x *= -1.
    //     }
    // } else if ball_transform.translation.x
    //     <= GAME_FRAME_POSITION.x - GAME_FRAME_WIDTH / 2. + BALL_SIZE / 2.
    // {
    //     if ball_vel.0.x < 0. {
    //         ball_vel.0.x *= -1.
    //     }
    // }


    if ball_transform.translation.x + BALL_SIZE/2. >= (GAME_FRAME_WIDTH/2. - 20.) - PAD_WIDTH/2. && ball_transform.translation.x - BALL_SIZE/2. <= (GAME_FRAME_WIDTH/2. - 20.) {
        if ball_transform.translation.y - BALL_SIZE/2. <= right_pad_transform.translation.y + pad_info.height/2. && ball_transform.translation.y + BALL_SIZE/2. >= right_pad_transform.translation.y - pad_info.height/2.{
            if ball_vel.0.x > 0. {
                ball_vel.0.x *= -1.
            }
        }
    }
    if ball_transform.translation.x - BALL_SIZE/2. <= (-GAME_FRAME_WIDTH/2. + 20.) + PAD_WIDTH/2. && ball_transform.translation.x + BALL_SIZE/2. >= (-GAME_FRAME_WIDTH/2. + 20.) {
        if ball_transform.translation.y - BALL_SIZE/2. <= left_pad_transform.translation.y + pad_info.height/2. && ball_transform.translation.y + BALL_SIZE/2. >= left_pad_transform.translation.y - pad_info.height/2.{
            if ball_vel.0.x < 0. {
                ball_vel.0.x *= -1.
            }
        }
    }


}

fn pad_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut left_pad_query: Query<&mut Transform, (With<LeftPad>, Without<RightPad>)>,
    mut right_pad_query: Query<&mut Transform, (With<RightPad>, Without<LeftPad>)>,
    pad_info: Res<PadInfo>
) {
    let mut left_pad_transform = left_pad_query.iter_mut().next().unwrap();
    let mut right_pad_transform = right_pad_query.iter_mut().next().unwrap();

    if keyboard_input.pressed(KeyCode::W) {
        left_pad_transform.translation.y += PAD_SPEED
    } else if keyboard_input.pressed(KeyCode::S) {
        left_pad_transform.translation.y -= PAD_SPEED
    }
    while left_pad_transform.translation.y > GAME_FRAME_HEIGHT/2. + GAME_FRAME_POSITION.y - pad_info.height/2. {
        left_pad_transform.translation.y -= 1.
    }
    while left_pad_transform.translation.y < -GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y + pad_info.height/2. {
        left_pad_transform.translation.y += 1.
    }


    if keyboard_input.pressed(KeyCode::Up) {
        right_pad_transform.translation.y += PAD_SPEED
    } else if keyboard_input.pressed(KeyCode::Down) {
        right_pad_transform.translation.y -= PAD_SPEED
    }
    while right_pad_transform.translation.y > GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y - pad_info.height/2. {
        right_pad_transform.translation.y -= 1.
    }
    while right_pad_transform.translation.y < -GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y + pad_info.height/2. {
        right_pad_transform.translation.y += 1.
    }
}

fn text_update(
    left_query: Query<&mut Text, (With(LeftScoreText), Without(RightScoreText))>,
    right_query: Query<&mut Text, (With(RightScoreText), Without(LeftScoreText))>,
    score: Res<Score>,
){
    for mut text in &mut left_query {
        text.sections[0].value = format!("{score.left_score}")
    }
    for mut text in &mut right_query {
        text.sections[0].value = format!("{score.right_score}")
    }
}

fn restart(
    mut ball_vel: ResMut<BallVelocity>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<LeftPad>, Without<RightPad>)>,
    mut score: ResMut<Score>,
) {
    let mut ball_transform = ball_query.single_mut();
    
    // if keyboard_input.just_pressed(KeyCode::R) {
        if ball_transform.translation.x >= GAME_FRAME_POSITION.x + GAME_FRAME_WIDTH / 2. {
            score.left_score += 1.;
            ball_transform.translation = Vec3::new(rand::thread_rng().gen_range(-10.0..=10.0), rand::thread_rng().gen_range(-100.0..=100.0), 3.);
            ball_vel.0 = Vec2::new(if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. }, if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. });
        } else if ball_transform.translation.x <= GAME_FRAME_POSITION.x - GAME_FRAME_WIDTH / 2. {  
            score.right_score += 1.;
            ball_transform.translation = Vec3::new(rand::thread_rng().gen_range(-10.0..=10.0), rand::thread_rng().gen_range(-100.0..=100.0), 3.);
            ball_vel.0 = Vec2::new(if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. }, if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. });
        }
    // }
}


fn print_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("FPS: {value:.2}")
            }
        }
    }
}
