use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    // input::keyboard,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowTheme, WindowResolution}, text::DEFAULT_FONT_HANDLE, audio::{VolumeLevel, Volume},
};
use rand::Rng;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct LeftPad;

#[derive(Component)]
struct RightPad;

#[derive(Resource)]
struct GameResources {
    ball_velocity: Vec2,
    ball_speed: f32,
    
    pad_height: f32,
    pad_speed: f32,

    left_score: u32,
    right_score: u32,
    
}


#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct LeftScoreText;

#[derive(Component)]
struct RightScoreText;


#[derive(Event)]
struct ScoreIncrementEvent;

const BALL_SPEED_INITIAL: f32 = 450.;
const BALL_SPEED_MAX: f32 = 900.;
const BALL_SPEED_INCREAMENT: f32 = 1.0;
const BALL_SIZE: f32 = 32.;

const GAME_FRAME_WIDTH: f32 = 700.;
const GAME_FRAME_HEIGHT: f32 = 450.;
const GAME_FRAME_THICKNESS: f32 = 5.;
const GAME_FRAME_POSITION: Vec2 = Vec2::new(0., -25.);

const PAD_WIDTH: f32 = 10.;
const PAD_SPEED_INITIAL: f32 = 10.;

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
        .insert_resource(GameResources{ ball_velocity: Vec2::new(1.0, 1.0), pad_height: 80., pad_speed: PAD_SPEED_INITIAL, left_score: 0, right_score: 0, ball_speed: BALL_SPEED_INITIAL})
        .insert_resource(ClearColor(Color::rgb(0.03, 0.03, 0.03)))
        .add_event::<ScoreIncrementEvent>()
        .add_systems(Startup, startup)
        .add_systems(Update, (print_fps, ball_movement, ball_collision, pad_movement, reset_with_score_increase, text_update.run_if(on_event::<ScoreIncrementEvent>()).after(reset_with_score_increase)))
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_resources: Res<GameResources>,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
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
                .with_scale(Vec3::new(PAD_WIDTH, game_resources.pad_height, 0.)),
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
                .with_scale(Vec3::new(PAD_WIDTH, game_resources.pad_height, 0.)),
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


    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                height: Val::Vh(100.),
                width: Val::Vw(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle { 
                        font: asset_server.load("fonts/bit5x3.ttf"), 
                        font_size: 64., 
                        color: Color::WHITE, 
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_background_color(Color::RED)
                .with_style(Style {
                    width: Val::Percent(25.),
                    ..Default::default()
                }),
                LeftScoreText,
            ));
        });

    // 왼쪽 스코어 텍스트
    // commands.spawn((
    //     TextBundle::from_section(
    //         "0",
    //         TextStyle { 
    //             font: asset_server.load("fonts/bit5x3.ttf"), 
    //             font_size: 64., 
    //             color: Color::WHITE, 
    //         },
    //     )
    //     .with_text_alignment(TextAlignment::Center)
    //     .with_background_color(Color::RED)
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         // left: Val::Px(window.width()/2. + GAME_FRAME_POSITION.x - GAME_FRAME_WIDTH/4.),
    //         display: Display::Flex,
    //         justify_content: JustifyContent::Center,
    //         ..Default::default()
    //     }),
    //     LeftScoreText,
    // ));
    
    // 오른쪽 스코어 텍스트
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
        .with_background_color(Color::BLUE)
        .with_style(Style {
            position_type: PositionType::Absolute,

            // left: Val::Px(window.width()/2. + GAME_FRAME_POSITION.x + GAME_FRAME_WIDTH/4.),
            top: Val::Px(20.),
            ..Default::default()
        }),
        RightScoreText,
    ));
    
    // 가운데 ':' 텍스트
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
            right: Val::Px(window.width()/2. + GAME_FRAME_POSITION.x),
            top: Val::Px(15.),
            ..Default::default()
        }),
    ));
}

fn ball_movement(
    mut query: Query<(&mut Ball, &mut Transform)>,
    mut game_resources: ResMut<GameResources>,
    time: Res<Time>,
) {
    if game_resources.ball_speed < BALL_SPEED_MAX {
        game_resources.ball_speed += BALL_SPEED_INCREAMENT;
        game_resources.pad_speed += ((PAD_SPEED_INITIAL * BALL_SPEED_MAX / BALL_SPEED_INITIAL) - PAD_SPEED_INITIAL) / ((BALL_SPEED_MAX - BALL_SPEED_INITIAL) / BALL_SPEED_INCREAMENT);
    } 

    // println!("{} | {}", game_resources.ball_speed, game_resources.pad_speed);


    let (_ball, mut transform) = query.single_mut();
    transform.translation += Vec3::new(game_resources.ball_velocity.x, game_resources.ball_velocity.y, 0.) * game_resources.ball_speed * time.delta_seconds();
}

fn ball_collision(
    mut game_resources: ResMut<GameResources>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<LeftPad>, Without<RightPad>)>,
    mut left_pad_query: Query<&mut Transform, (With<LeftPad>, Without<RightPad>, Without<Ball>)>,
    mut right_pad_query: Query<&mut Transform, (With<RightPad>, Without<LeftPad>, Without<Ball>)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let ball_transform = ball_query.single_mut();
    let left_pad_transform = left_pad_query.single_mut();
    let right_pad_transform = right_pad_query.single_mut();

    if ball_transform.translation.y
        >= GAME_FRAME_POSITION.y + GAME_FRAME_HEIGHT / 2. - BALL_SIZE / 2.
    {
        if game_resources.ball_velocity.y > 0. {
            game_resources.ball_velocity.y *= -1.
        }
    } else if ball_transform.translation.y
        <= GAME_FRAME_POSITION.y - GAME_FRAME_HEIGHT / 2. + BALL_SIZE / 2.
    {
        if game_resources.ball_velocity.y < 0. {
            game_resources.ball_velocity.y *= -1.
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
        if ball_transform.translation.y - BALL_SIZE/2. <= right_pad_transform.translation.y + game_resources.pad_height/2. && ball_transform.translation.y + BALL_SIZE/2. >= right_pad_transform.translation.y - game_resources.pad_height/2.{
            if game_resources.ball_velocity.x > 0. {
                game_resources.ball_velocity.x *= -1.;
                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("sounds/HitSound.mp3"),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            ..Default::default()
                        }
                });
            }
        }
    }
    if ball_transform.translation.x - BALL_SIZE/2. <= (-GAME_FRAME_WIDTH/2. + 20.) + PAD_WIDTH/2. && ball_transform.translation.x + BALL_SIZE/2. >= (-GAME_FRAME_WIDTH/2. + 20.) {
        if ball_transform.translation.y - BALL_SIZE/2. <= left_pad_transform.translation.y + game_resources.pad_height/2. && ball_transform.translation.y + BALL_SIZE/2. >= left_pad_transform.translation.y - game_resources.pad_height/2.{
            if game_resources.ball_velocity.x < 0. {
                game_resources.ball_velocity.x *= -1.;
                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("sounds/HitSound.mp3"),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            ..Default::default()
                        }
                });
            }
        }
    }


}

fn pad_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut left_pad_query: Query<&mut Transform, (With<LeftPad>, Without<RightPad>)>,
    mut right_pad_query: Query<&mut Transform, (With<RightPad>, Without<LeftPad>)>,
    game_resources: Res<GameResources>
) {
    let mut left_pad_transform = left_pad_query.iter_mut().next().unwrap();
    let mut right_pad_transform = right_pad_query.iter_mut().next().unwrap();

    if keyboard_input.pressed(KeyCode::W) {
        left_pad_transform.translation.y += game_resources.pad_speed
    } else if keyboard_input.pressed(KeyCode::S) {
        left_pad_transform.translation.y -= game_resources.pad_speed
    }
    while left_pad_transform.translation.y > GAME_FRAME_HEIGHT/2. + GAME_FRAME_POSITION.y - game_resources.pad_height/2. {
        left_pad_transform.translation.y -= 1.
    }
    while left_pad_transform.translation.y < -GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y + game_resources.pad_height/2. {
        left_pad_transform.translation.y += 1.
    }


    if keyboard_input.pressed(KeyCode::Up) {
        right_pad_transform.translation.y += game_resources.pad_speed
    } else if keyboard_input.pressed(KeyCode::Down) {
        right_pad_transform.translation.y -= game_resources.pad_speed
    }
    while right_pad_transform.translation.y > GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y - game_resources.pad_height/2. {
        right_pad_transform.translation.y -= 1.
    }
    while right_pad_transform.translation.y < -GAME_FRAME_HEIGHT/2.+ GAME_FRAME_POSITION.y + game_resources.pad_height/2. {
        right_pad_transform.translation.y += 1.
    }
}

fn text_update(
    mut left_query: Query<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right_query: Query<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
    game_resources: Res<GameResources>,
){ 
    for mut text in &mut left_query {
        text.sections[0].value = game_resources.left_score.to_string()
    }
    for mut text in &mut right_query {
        text.sections[0].value = game_resources.right_score.to_string()
    }
}

fn reset_with_score_increase(
    mut game_resources: ResMut<GameResources>,
    mut ball_query: Query<&mut Transform, (With<Ball>, Without<LeftPad>, Without<RightPad>)>,
    mut score_events: EventWriter<ScoreIncrementEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut ball_transform = ball_query.single_mut();
    
        if ball_transform.translation.x >= GAME_FRAME_POSITION.x + GAME_FRAME_WIDTH / 2. {
            game_resources.left_score += 1;
            ball_transform.translation = Vec3::new(rand::thread_rng().gen_range(-10.0..=10.0), rand::thread_rng().gen_range(-100.0..=100.0), 3.);
            game_resources.ball_velocity = Vec2::new(if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. }, if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. });
            game_resources.ball_speed = BALL_SPEED_INITIAL;
            game_resources.pad_speed = PAD_SPEED_INITIAL;
            commands.spawn(
                AudioBundle {
                    source: asset_server.load("sounds/Coin.mp3"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Absolute(VolumeLevel::new(0.1)),
                        ..Default::default()
                    }
            });
            score_events.send(ScoreIncrementEvent);
        } else if ball_transform.translation.x <= GAME_FRAME_POSITION.x - GAME_FRAME_WIDTH / 2. {  
            game_resources.right_score += 1;
            ball_transform.translation = Vec3::new(rand::thread_rng().gen_range(-10.0..=10.0), rand::thread_rng().gen_range(-100.0..=100.0), 3.);
            game_resources.ball_velocity = Vec2::new(if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. }, if rand::thread_rng().gen_bool(0.5) { 1. } else { -1. });
            game_resources.ball_speed = BALL_SPEED_INITIAL;
            game_resources.pad_speed = PAD_SPEED_INITIAL;
            commands.spawn(
                AudioBundle {
                    source: asset_server.load("sounds/Coin.mp3"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Absolute(VolumeLevel::new(0.1)),
                        ..Default::default()
                    }
            });
            score_events.send(ScoreIncrementEvent);
        }
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
