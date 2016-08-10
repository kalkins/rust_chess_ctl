extern crate chess;

use std::io::{self, Write};
use chess::*;

fn main() {
    println!("Welcome to command line chess. Execute a move by typing in the algebraic notation for the move and hitting Return.");
    let mut game = Game::new();
    let mut color = Color::Black;
    let mut input: String;

    loop {
        color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        loop {
            input = String::new();
            print!("{}: ", color);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            match game.an_to_move(&input.trim(), color) {
                Some(v) => {
                    game.move_pieces(&v);
                    break;
                },
                None => println!("Invalid move"),
            }
        }

        if let Some(v) = game.check_victory() {
            match v.0 {
                Victory::Checkmate => println!("{} won by checkmate!", v.1),
                Victory::Stalemate => println!("Stalemate"),
                Victory::Draw      => println!("Draw"),
            }
            break;
        }
    }
}
