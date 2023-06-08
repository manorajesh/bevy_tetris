use crate::tetrominoe::Tetrominoe;
use crate::{
    gamestate::GameState,
    tetrominoe::{State, TColor},
};

pub const EMP: char = '.';

pub fn init(width: usize, height: usize) -> Vec<Vec<Tetrominoe>> {
    let mut display: Vec<Vec<Tetrominoe>> = Vec::new();

    // generation
    for _ in 0..height {
        display.push(vec![Tetrominoe::default(); width]);
    }
    display
}

pub fn gravity(gs: &mut GameState) -> bool {
    let prev_display = gs.display.clone();
    for row in (0..gs.display.len()).rev() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col].game_state == State::Active {
                if row == gs.display.len() - 1
                    || gs.display[row + 1][col].game_state == State::Landed
                {
                    gs.display = prev_display;
                    landed(gs);
                    let game_over = new_piece(gs, None);
                    return game_over;
                }

                gs.display[row + 1][col] = gs.display[row][col];
                gs.display[row][col] = Tetrominoe::new(None, None);
            }
        }
    }
    gs.active_piece.row += 1;
    false
}

pub fn handle_input(gs: &mut GameState, key: char) {
    let prev_display = gs.display.clone();
    match key {
        'l' => {
            for row in (0..gs.display.len()).rev() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col].game_state == State::Active {
                        if col == 0 || gs.display[row][col - 1].game_state == State::Landed {
                            gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col - 1] = gs.display[row][col];
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            if gs.active_piece.col > 0 {
                gs.active_piece.col -= 1;
            }
        }

        'r' => {
            for row in (0..gs.display.len()).rev() {
                for col in (0..gs.display[row].len()).rev() {
                    if gs.display[row][col].game_state == State::Active {
                        if col == gs.display[row].len() - 1
                            || gs.display[row][col + 1].game_state == State::Landed
                        {
                            gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col + 1] = gs.display[row][col];
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }
            gs.active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while gs.display[0][gs.display[0].len() / 2].game_state == State::Empty {
                gravity(gs);
            }
        }

        'd' => {
            gravity(gs);
        }

        'u' => {
            // let prev_display = gs.display.clone();
            let prev_piece = gs.active_piece;

            // rotate piece
            gs.active_piece.rotate();
            if gs.active_piece.row + 4 > gs.display.len() {
                gs.active_piece.row = gs.display.len() - 4;
            }

            if gs.active_piece.col + 4 > gs.display[0].len() {
                gs.active_piece.col = gs.display[0].len() - 4;
            }

            // clear piece and replace with new rotated piece
            for row in 0..gs.display.len() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col].game_state == State::Active {
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            for row in gs.active_piece.row..gs.active_piece.row + 4 {
                for col in gs.active_piece.col..gs.active_piece.col + 4 {
                    if gs.display[row][col].game_state == State::Landed {
                        gs.display = prev_display;
                        gs.active_piece = prev_piece;
                        return;
                    }

                    if gs.active_piece.shape[row - gs.active_piece.row][col - gs.active_piece.col]
                        == 'a'
                    {
                        gs.display[row][col] =
                            Tetrominoe::new(Some(State::Active), Some(gs.active_piece.color));
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(gs: &mut GameState, desired_piece: Option<char>) -> bool {
    let half_width = gs.display[0].len() / 2;

    // game over
    if gs.display[0][half_width].game_state != State::Empty {
        return true;
    }

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(gs));
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[3][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
        }
        'J' => {
            //  J
            //  J
            // JJ
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[2][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
        }
        'L' => {
            // L
            // L
            // LL
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[2][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
        }
        'O' => {
            // OO
            // OO
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[0][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[1][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
        }
        'S' => {
            // SS
            //  SS
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[0][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[1][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
        }
        'T' => {
            // T
            // TT
            // T
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
        }
        'Z' => {
            //  ZZ
            // ZZ
            gs.display[0][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[1][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
        }
        _ => panic!("unknown picece: {}", piece),
    }
    gs.active_piece.set(piece);
    gs.active_piece.set_pos(0, half_width - 1);
    false
}

pub fn landed(gs: &mut GameState) {
    for row in &mut gs.display {
        for ch in row {
            if ch.game_state == State::Active {
                ch.game_state = State::Landed;
            }
        }
    }
}

pub fn full_line(gs: &mut GameState) {
    let mut lines: usize = 0;
    'outer: for row in (0..gs.display.len()).rev() {
        for ch in &gs.display[row] {
            if ch.game_state != State::Landed {
                continue 'outer;
            }
        }
        gs.display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        gs.display
            .insert(0, vec![Tetrominoe::default(); gs.display[0].len()]); // add new line at the top
    }

    match lines {
        1 => gs.gamescore.score += 40 * (gs.gamescore.level + 1),
        2 => gs.gamescore.score += 100 * (gs.gamescore.level + 1),
        3 => gs.gamescore.score += 300 * (gs.gamescore.level + 1),
        4 => gs.gamescore.score += 1200 * (gs.gamescore.level + 1),
        _ => (),
    }

    gs.gamescore.level = gs.gamescore.score / 1000;
}

pub fn ghost_piece(gs: &mut GameState) {
    for row in 0..gs.display.len() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col].game_state == State::Ghost {
                gs.display[row][col].game_state = State::Empty;
            }
        }
    }

    let mut ghost = gs.clone();

    gravity_until_new_piece(&mut ghost);

    for row in 0..ghost.display.len() {
        for col in 0..ghost.display[row].len() {
            if ghost.display[row][col].game_state == State::Active
                && gs.display[row][col].game_state == State::Empty
            {
                gs.display[row][col].game_state = State::Ghost;
            }
        }
    }
}

fn gravity_until_new_piece(gs: &mut GameState) {
    let mut prev_display = gs.display.clone();
    gravity(gs);
    while gs.display[0][gs.display[0].len() / 2].game_state == State::Empty {
        prev_display = gs.display.clone();
        gravity(gs);
    }
    gs.display = prev_display;
}

pub fn hold(gs: &mut GameState) {
    // clear piece
    for row in gs.display.iter_mut() {
        for col in row.iter_mut() {
            if col.game_state == State::Active {
                col.game_state = State::Empty;
            }
        }
    }

    // hold piece
    if let Some(hold) = &gs.hold_piece {
        let prev_piece = gs.active_piece;
        new_piece(gs, Some(hold.ptype));
        gs.hold_piece = Some(prev_piece);
    } else {
        gs.hold_piece = Some(gs.active_piece);
        new_piece(gs, None);
    }
}

fn get_next_piece(gs: &mut GameState) -> char {
    let temp = gs.next_piece.ptype;
    gs.next_piece = Tetrominoe::random(&mut gs.bag);
    temp
}
