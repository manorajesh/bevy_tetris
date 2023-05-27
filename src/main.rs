// Tetris

mod gamescore;
mod gamestate;
mod tetlib;
mod tetrominoe;

use bevy::prelude::*;

use std::{
    thread::sleep,
    time::Duration,
};

use gamestate::GameState;
use tetlib::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

fn setup(mut commands: Commands) {

}

fn main() {
    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 40;
    const LEVEL_MULT: f64 = 0.85;

    let mut gs = GameState::new(WIDTH, HEIGHT);

    // loop for new game
    loop {
        // game loop
        loop {
            let prev_display = gs.display.clone();

            // handle input
            let key = get_input();

            // quit
            if key == 'q' {
                break;
            }

            if key == 'p' {
                let mut key = get_input();
                put_text(WIDTH as u16, HEIGHT as u16, "P A U S E D");
                while key != 'p' && key != 'q' {
                    key = get_input();
                    sleep(Duration::from_millis(10));
                }
            }

            // gravity
            if gs.counter
                >= (GRAV_TICK as f64 * LEVEL_MULT.powf(gs.gamescore.level as f64)) as usize
            {
                if gravity(&mut gs) {
                    gs.is_game_over = true;
                    break;
                }
                gs.counter = if gs.gamescore.level < MAX_LEVEL {
                    0
                } else {
                    100
                };
            }

            // handle input
            handle_input(&mut gs, key);

            // hold piece
            if key == 'c' {
                hold(&mut gs);
            }

            // full line
            full_line(&mut gs);

            // ghost piece
            ghost_piece(&mut gs);

            // check if gs.display was changed
            let is_updated = gs.display != prev_display || gs.is_game_over;

            // render
            render(&mut gs, is_updated, &"██".to_string(), &true);
            sleep(Duration::from_millis(10));
            gs.counter += 1;
        }
        gs = GameState::new(WIDTH, HEIGHT);
    }
}

fn path_exists(path: &String) -> bool {
    std::path::Path::new(path).exists()
}
