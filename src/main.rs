extern crate chess;
extern crate pancurses;

use chess::*;
use pancurses::*;

fn main() {
    let mut window = pancurses::initscr();
    let mut game = Game::new();
    let mut color = Color::Black;
    let mut turn = 0;
    let mut scroll: usize;
    let mut base: String = String::new();
    let mut input: String;
    let mut invalid: bool;
    let mut proper: String;
    let mut buffer: Vec<Option<String>> = vec![Some("Welcome to command line chess. \
        Execute a move by typing in the algebraic notation for the move and hitting Return.".to_string()), None, None];

    pancurses::noecho();
    window.nodelay(false);

    loop {
        color = match color {
            Color::White => Color::Black,
            Color::Black => {
                turn += 1;
                Color::White
            },
        };
        if color == Color::White {
            base = format!("{}: ", turn);
        }

        invalid = false;
        loop {
            scroll = 0;
            input = String::new();
            buffer.push(None);
            loop {
                buffer.pop();
                let mut s = base.clone();
                s.push_str(&input);
                buffer.push(Some(s));
                draw(&mut window, &buffer, scroll, &game);
                if let Some(inp) = window.getch() {
                    if let Input::Character(ch) = inp {
                        match ch {
                            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => {
                                scroll = 0;
                                if ch == 'v' {
                                    buffer.push(Some("test".to_string()));
                                } else {
                                    input.push(ch);
                                }
                            },
                            '\u{0008}' | '\u{007f}' => {input.pop();},
                            '\u{001b}' => {
                                window.getch();
                                match window.getch().unwrap() {
                                    Input::Character('A') => {
                                        let mut height: usize = 0;
                                        for _ in 0..window.get_max_y() {
                                            height += 1;
                                        }
                                        if buffer.len() > height + scroll {
                                            scroll += 1;
                                        }
                                    },
                                    Input::Character('B') => {
                                        if scroll > 0 {
                                            scroll -= 1;
                                        }
                                    },
                                    _ => panic!("Invalid input '{}'", ch),
                                }
                            },
                            '\n' | '\t' | ' ' => break,
                            _ => panic!("Invalid input '{}'", ch),
                        }
                    }
                }
            }

            match game.an_to_move(&input.trim(), color) {
                Some(v) => {
                    if invalid {
                        let len = buffer.len();
                        buffer.remove(len-2);
                    }

                    proper = game.move_to_an(&v, true);
                    buffer.pop();
                    if color == Color::Black {
                        let mut s = base.clone();
                        s.push_str(&proper);
                        buffer.push(Some(s));
                    } else {
                        base.push_str(&proper);
                        for _ in base.len()..15 {
                            base.push(' ');
                        }
                    }

                    game.move_pieces(&v);
                    break;
                },
                None => {
                    if invalid {
                        buffer.pop();
                    }
                    invalid = true;
                    buffer.pop();
                    buffer.push(Some("Invalid move".to_string()));
                },
            };
        }

        if let Some(v) = game.check_victory() {
            buffer.push(None);
            buffer.push(Some(match v.0 {
                Victory::Checkmate => {
                    let mut s = String::new();
                    s.push_str(match v.1 {
                        Color::White => "White",
                        Color::Black => "Black",
                    });
                    s.push_str(" won by checkmate!");
                    s
                },
                Victory::Stalemate => "Stalemate".to_string(),
                Victory::Draw      => "Draw".to_string(),
            }));

            buffer.push(None);
            buffer.push(Some("Press any key to quit.".to_string()));

            draw(&mut window, &buffer, scroll, &game);
            break;
        }
    }

    window.getch();

    pancurses::delwin(window);
    pancurses::endwin();
}

fn draw(window: &mut pancurses::Window, buffer: &[Option<String>], index: usize, game: &Game) {
    let len = buffer.len();
    let mut line = 0;
    let width = window.get_max_x();
    let mut height: usize = 0;
    for _ in 0..window.get_max_y() {
        height += 1;
    }

    let start: usize = if len > height {
        len - height - index
    } else {
        0
    };

    window.erase();
    for i in 0..height {
        if let Some(ref v) = buffer[start + i] {
            window.addstr(v);
        }
        if start + i == len - 1 {
            break;
        } else if i != height - 1 {
            line += 1;
            window.mv(line, 0);
        }
    }

    if height > 8 && width > 40 {
        let start_x = width - (width / 4 + 4);
        let start_y = window.get_max_y() / 2 - 4;
        let board = game.board_to_string();
        let mut i: usize = 0;
        let mut y = start_y;
        let mut x;

        for tmp in 0..8 {
            x = start_x;
            window.mvaddch(y, x-2, match tmp {
                0 => '8',
                1 => '7',
                2 => '6',
                3 => '5',
                4 => '4',
                5 => '3',
                6 => '2',
                7 => '1',
                _ => panic!(),
            });
            for _ in 0..8 {
                window.mvaddch(y, x, board.chars().nth(i).unwrap());
                x += 1;
                i += 1;
            }
            y += 1;
            i += 1;
        }
        window.mvaddstr(y+1, start_x, "ABCDEFGH");

        x = 0;
        for _ in 0..buffer.last().unwrap().as_ref().unwrap().len() {
            x += 1;
        }
        window.mv(line, x);
    }

    window.refresh();
}
