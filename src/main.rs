// Tetris

use bevy::prelude::*;
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Block, SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-400., 300., 0.)),
        ..default()
    }));
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
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new((-200 + col.0 as i32 * 20) as f32, (200 - row.0 as i32 * 20) as f32, 0.)),
                        ..default()
                    }));
                    // print!("A")
                }, 
                State::Ghost => {
                    commands.spawn((Block, SpriteBundle {
                        sprite: Sprite {
                            color: Color::Rgba { red: 1., green: 1., blue: 1., alpha: 0.1 },
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new((-200 + col.0 as i32 * 20) as f32, (200 - row.0 as i32 * 20) as f32, 0.)),
                        ..default()
                    }));
                    // print!("G")
                },
                _ => {
                    // print!(".")
                }
            }
        }
        println!()
    }
}

fn move_sprites(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::new(10, 20))
        .insert_resource(GameTimer(Timer::from_seconds(0.4, TimerMode::Repeating)))
        // .add_simple_outer_schedule()
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