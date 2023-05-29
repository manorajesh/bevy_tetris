// Tetris

use bevy::{prelude::*, window::PresentMode, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use gamestate::GameState;
use tetlib::*;
use tetrominoe::State;

mod tetlib;
mod tetrominoe;
mod gamestate;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Component)]
struct Block;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

const LEFT: i32 = -88;
const TOP: i32 = 200;

const BLOCK_SIZE: f32 = 20.0;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(Camera2dBundle::default());
    audio.play_with_settings(
        asset_server.load("music/korobeiniki.ogg"),
        PlaybackSettings::LOOP.with_volume(0.5),
    );

    // top
    for i in 0..WIDTH {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new((LEFT + i as i32 * BLOCK_SIZE as i32) as f32, TOP as f32+BLOCK_SIZE, 0.)),
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
            transform: Transform::from_translation(Vec3::new((LEFT + i as i32 * BLOCK_SIZE as i32) as f32, TOP as f32 - HEIGHT as f32*BLOCK_SIZE, 0.)),
            ..default()
        });
    }

    // left
    for i in 0..=HEIGHT+1 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                // color: col.1.as_color(),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(LEFT as f32-BLOCK_SIZE, (TOP - i as i32 * BLOCK_SIZE as i32) as f32+BLOCK_SIZE, 0.)),
            ..default()
        });
    }

    // right
    for i in 0..=HEIGHT+1 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("blocks/gray.png"),
            sprite: Sprite {
                // color: col.1.as_color(),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(LEFT as f32+WIDTH as f32 *BLOCK_SIZE, (TOP - i as i32 * BLOCK_SIZE as i32) as f32+BLOCK_SIZE, 0.)),
            ..default()
        });
    }
}

fn gravity_system(mut gs: ResMut<GameState>, mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        gravity(&mut *gs);
    }
}

fn handle_input_system(mut gs: ResMut<GameState>, keyboard_input: Res<Input<KeyCode>>)  {
    if keyboard_input.just_pressed(KeyCode::Left) {
        handle_input(&mut *gs, 'l');
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        handle_input(&mut *gs, 'r');
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        handle_input(&mut *gs, 's');
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        handle_input(&mut *gs, 'd');
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        handle_input(&mut *gs, 'u');
    } else if keyboard_input.just_pressed(KeyCode::C) {
        hold(&mut *gs);
    }
}

fn ghost_piece_system(mut gs: ResMut<GameState>) {
        ghost_piece(&mut *gs);
}

fn full_line_system(mut gs: ResMut<GameState>) {
    full_line(&mut *gs);
}

fn render_system(gs: Res<GameState>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in gs.display.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            match col.1.game_state {
                State::Landed | State::Active => {
                    commands.spawn((Block, SpriteBundle {
                        texture: asset_server.load(col.1.as_color()),
                        sprite: Sprite {
                            // color: col.1.as_color(),
                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new((LEFT + col.0 as i32 * BLOCK_SIZE as i32) as f32, (TOP - row.0 as i32 * BLOCK_SIZE as i32) as f32, 0.)),
                        ..default()
                    }));
                    // print!("A")
                }, 
                State::Ghost => {
                    commands.spawn((Block, SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 0.1 },
                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new((LEFT + col.0 as i32 * BLOCK_SIZE as i32) as f32, (TOP - row.0 as i32 * BLOCK_SIZE as i32) as f32, 0.)),
                        ..default()
                    }));
                    // print!("G")
                },
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
    .add_plugins(DefaultPlugins.set(WindowPlugin {  
        primary_window: Some(Window {  
        title: "Tetris".into(),  
        resolution: (500., 600.).into(),  
        present_mode: PresentMode::AutoVsync,  
        fit_canvas_to_parent: true,  
        prevent_default_event_handling: false,  
        ..default()  
        }),  
        ..default()  
        }))
        .add_plugin(LogDiagnosticsPlugin::default())  
        .add_plugin(FrameTimeDiagnosticsPlugin)  
        .insert_resource(GameState::new(10, 20))
        .insert_resource(GameTimer(Timer::from_seconds(0.4, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_systems(
            (
                gravity_system,
                handle_input_system,
                ghost_piece_system,
                full_line_system,
                render_system,
                move_sprites,
            )
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}