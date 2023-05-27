// Tetris

use bevy::prelude::*;
use gamestate::GameState;
use tetlib::*;

mod tetlib;
mod tetrominoe;
mod gamestate;

#[derive(Resource)]
struct GameTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
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

fn ghost_piece_system(mut gs: ResMut<GameState>, mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        ghost_piece(&mut *gs);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::new(10, 20))
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_system(gravity_system)
        .add_system(handle_input_system)
        .add_system(ghost_piece_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}