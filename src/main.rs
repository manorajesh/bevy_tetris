// Tetris

use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::Stopwatch,
    window::PresentMode,
};
use gamestate::GameState;
use tetlib::*;
use tetrominoe::{State, Tetrominoe};

mod bag;
mod gamescore;
mod gamestate;
mod tetlib;
mod tetrominoe;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Level;

#[derive(Component)]
struct WatchText;

#[derive(Resource)]
struct Watch {
    time: Stopwatch,
}

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

const LEFT: i32 = -110;
const TOP: i32 = 200;

const TEXT_TOP_PADDING: f32 = -135.0;
const LEFT_TEXT_PADDING: f32 = 165.0;
const RIGHT_TEXT_PADDING: f32 = 530.0;

const BLOCK_SIZE: f32 = 20.0;
const FONT_SIZE: f32 = 23.0;

const VOLUME: f32 = 0.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(Camera2dBundle::default());
    audio.play_with_settings(
        asset_server.load("music/korobeiniki.ogg"),
        PlaybackSettings::LOOP.with_volume(VOLUME),
    );

    // Hold text
    commands.spawn(
        TextBundle::from_sections([TextSection {
            value: "HOLD".to_string(),
            style: TextStyle {
                font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
        }])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(TOP as f32 + TEXT_TOP_PADDING),
                left: Val::Px(LEFT as f32 + LEFT_TEXT_PADDING),
                ..default()
            },
            ..default()
        }),
    );

    // Scoreboard
    // Score
    commands.spawn((
        Score,
        TextBundle::from_sections([
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(TOP as f32 + TEXT_TOP_PADDING),
                left: Val::Px(LEFT as f32 + RIGHT_TEXT_PADDING),
                ..default()
            },
            ..default()
        }),
    ));

    commands.spawn((
        Level,
        TextBundle::from_sections([
            TextSection::new(
                "LEVEL: ",
                TextStyle {
                    font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(TOP as f32 + TEXT_TOP_PADDING + 50.),
                left: Val::Px(LEFT as f32 + RIGHT_TEXT_PADDING),
                ..default()
            },
            ..default()
        }),
    ));

    commands.spawn((
        WatchText,
        TextBundle::from_sections([
            TextSection::new(
                "TIME: ",
                TextStyle {
                    font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("font/Nineteen-Ninety-Seven.otf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(TOP as f32 + TEXT_TOP_PADDING + 100.),
                left: Val::Px(LEFT as f32 + RIGHT_TEXT_PADDING),
                ..default()
            },
            ..default()
        }),
    ));

    // top
    for i in 0..WIDTH {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (LEFT + i as i32 * BLOCK_SIZE as i32) as f32,
                TOP as f32 + BLOCK_SIZE,
                0.,
            )),
            ..default()
        });
    }

    // bottom
    for i in 0..WIDTH {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                // color: col.1.as_color(),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (LEFT + i as i32 * BLOCK_SIZE as i32) as f32,
                TOP as f32 - HEIGHT as f32 * BLOCK_SIZE,
                0.,
            )),
            ..default()
        });
    }

    // left
    for i in 0..=HEIGHT + 1 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                // color: col.1.as_color(),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                LEFT as f32 - BLOCK_SIZE,
                (TOP - i as i32 * BLOCK_SIZE as i32) as f32 + BLOCK_SIZE,
                0.,
            )),
            ..default()
        });
    }

    // right
    for i in 0..=HEIGHT + 1 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                // color: col.1.as_color(),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                LEFT as f32 + WIDTH as f32 * BLOCK_SIZE,
                (TOP - i as i32 * BLOCK_SIZE as i32) as f32 + BLOCK_SIZE,
                0.,
            )),
            ..default()
        });
    }
}

fn gravity_system(
    mut gs: ResMut<GameState>,
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if timer.0.tick(time.delta()).just_finished() && gravity(&mut gs) {
        app_exit_events.send(AppExit);
    }
}

fn handle_input_system(mut gs: ResMut<GameState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        handle_input(&mut gs, 'l');
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        handle_input(&mut gs, 'r');
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        handle_input(&mut gs, 's');
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        handle_input(&mut gs, 'd');
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        handle_input(&mut gs, 'u');
    } else if keyboard_input.just_pressed(KeyCode::C) {
        hold(&mut gs);
    }
}

fn ghost_piece_system(mut gs: ResMut<GameState>) {
    ghost_piece(&mut gs);
}

fn full_line_system(mut gs: ResMut<GameState>) {
    full_line(&mut gs);
}

fn update_score_system(gs: Res<GameState>, mut query: Query<&mut Text, With<Score>>) {
    let mut text = query.single_mut();
    text.sections[1].value = gs.gamescore.score.to_string();
    text.sections[1].style.font_size = FONT_SIZE - (text.sections[1].value.len() / 4) as f32 * 5.;
}

fn update_level_system(gs: Res<GameState>, mut query: Query<&mut Text, With<Level>>) {
    let mut text = query.single_mut();
    text.sections[1].value = gs.gamescore.level.to_string();
    text.sections[1].style.font_size = FONT_SIZE - (text.sections[1].value.len() / 4) as f32 * 5.;
}

fn update_stopwatch_system(
    time: Res<Time>,
    mut stopwatch: ResMut<Watch>,
    mut query: Query<&mut Text, With<WatchText>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = format!(
        "{}:{:02}",
        stopwatch.time.elapsed().as_secs() / 60,
        stopwatch.time.elapsed().as_secs() % 60
    );
    text.sections[1].style.font_size = FONT_SIZE - (text.sections[1].value.len() / 7) as f32 * 5.;
    stopwatch.time.tick(time.delta());
}

fn render_next(gs: Res<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in 0..gs.next_piece.shape.len() {
        for col in 0..gs.next_piece.shape[row].len() {
            if gs.next_piece.shape[row][col] == 'a' {
                commands.spawn((
                    Block,
                    SpriteBundle {
                        texture: asset_server.load(gs.next_piece.as_color()),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            (LEFT + col as i32 * BLOCK_SIZE as i32) as f32 + 270.,
                            (TOP - row as i32 * BLOCK_SIZE as i32) as f32 - 125.,
                            0.,
                        )),
                        ..default()
                    },
                ));
            }
        }
    }
}

fn render_hold(gs: Res<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    match &gs.hold_piece {
        Some(piece) => {
            let mut blank = Tetrominoe::new(None, None);
            let upright = blank.set(piece.ptype);
            for row in 0..upright.shape.len() {
                for col in 0..upright.shape[row].len() {
                    if upright.shape[row][col] == 'a' {
                        commands.spawn((
                            Block,
                            SpriteBundle {
                                texture: asset_server.load(upright.as_color()),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    (LEFT + col as i32 * BLOCK_SIZE as i32) as f32 - 119.,
                                    (TOP - row as i32 * BLOCK_SIZE as i32) as f32 - 20.,
                                    0.,
                                )),
                                ..default()
                            },
                        ));
                    }
                }
            }
        }

        None => (),
    }
}

fn render_system(gs: Res<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in gs.display.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            match col.1.game_state {
                State::Landed | State::Active => {
                    commands.spawn((
                        Block,
                        SpriteBundle {
                            texture: asset_server.load(col.1.as_color()),
                            sprite: Sprite {
                                // color: col.1.as_color(),
                                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                (LEFT + col.0 as i32 * BLOCK_SIZE as i32) as f32,
                                (TOP - row.0 as i32 * BLOCK_SIZE as i32) as f32,
                                0.,
                            )),
                            ..default()
                        },
                    ));
                    // print!("A")
                }
                State::Ghost => {
                    commands.spawn((
                        Block,
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 1.,
                                    green: 1.,
                                    blue: 1.,
                                    alpha: 0.1,
                                },
                                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                (LEFT + col.0 as i32 * BLOCK_SIZE as i32) as f32,
                                (TOP - row.0 as i32 * BLOCK_SIZE as i32) as f32,
                                0.,
                            )),
                            ..default()
                        },
                    ));
                    // print!("G")
                }
                _ => {
                    // print!(".")
                }
            }
        }
        // println!()
    }
}

fn move_sprites(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris".into(),
                resolution: (600., 600.).into(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .insert_resource(GameState::new(10, 20))
        .insert_resource(Watch {
            time: Stopwatch::new(),
        })
        .insert_resource(GameTimer(Timer::from_seconds(0.4, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_systems((
            gravity_system,
            handle_input_system,
            ghost_piece_system,
            full_line_system,
            update_score_system,
            update_level_system,
            update_stopwatch_system,
            render_hold,
            render_next,
            render_system.after(handle_input_system),
            move_sprites,
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}
