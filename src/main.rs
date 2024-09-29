use rand::Rng;
use std::io;
#[derive(PartialEq)]
enum Player {
    X,
    O,
}
#[derive(Debug)]
enum Result {
    Xwin,
    Owin,
    Draw,
    Ongoing,
}
#[derive(Clone)]
struct Board {
    board: [i8; 9],
}
impl Board {
    fn show_diagram(&self) {
        println!("1 2 3");
        println!("4 5 6");
        println!("7 8 9");
    }

    fn get_legal_moves(&self) -> Vec<i8> {
        let mut return_list: Vec<i8> = vec![];
        for (i, x) in self.board.iter().enumerate() {
            if *x == 0 {
                return_list.push((i + 1) as i8);
            }
        }
        return return_list;
    }
    fn ai_move(&self, turn: &Player) -> i8 {
        let legal_moves = self.get_legal_moves();
        let legal_game_moves_len = legal_moves.len();
        let mut rng = rand::thread_rng();
        let buffer_move: i8 = 0;
        for o in legal_moves.iter() {
            let mut buffer_board = self.clone();
            buffer_board.make_move(*o as usize, &turn);
            match buffer_board.is_a_win() {
                Result::Xwin => match turn {
                    Player::X => return *o,
                    Player::O => (),
                },
                Result::Owin => match turn {
                    Player::O => return *o,
                    Player::X => (),
                },
                _ => (),
            }
        }

        for o in legal_moves.iter() {
            let oponents_turn: Player;
            match turn {
                Player::X => oponents_turn = Player::O,
                Player::O => oponents_turn = Player::X,
            }
            let mut buffer_board = self.clone();
            buffer_board.make_move(*o as usize, &oponents_turn);
            match buffer_board.is_a_win() {
                Result::Xwin => match turn {
                    Player::O => return *o,
                    Player::X => (),
                },
                Result::Owin => match turn {
                    Player::X => return *o,
                    Player::O => (),
                },
                _ => (),
            }
        }

        if buffer_move == 0 {
            return legal_moves[rng.gen_range(0..legal_game_moves_len)];
        } else {
            return buffer_move;
        }
    }
    fn is_a_win(&self) -> Result {
        let win_condition_list: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        let mut result: Result = Result::Ongoing;
        for i in win_condition_list.iter() {
            let first_index: usize = i[0];
            let secound_index: usize = i[1];
            let third_index: usize = i[2];
            if (self.board[first_index] == self.board[secound_index])
                && (self.board[secound_index] == self.board[third_index])
                && (self.board[first_index] != 0)
            {
                if self.board[first_index] == 1 {
                    result = Result::Xwin
                } else {
                    result = Result::Owin
                }
                break;
            } else if self.get_legal_moves().is_empty() {
                result = Result::Draw
            } else {
                result = Result::Ongoing
            }
        }
        return result;
    }
    fn show_board(&self) {
        for (i, x) in self.board.iter().enumerate() {
            if *x == 1 {
                print!("x")
            } else if *x == 2 {
                print!("o")
            } else {
                print!("#")
            }
            if (i + 1) % 3 == 0 {
                println!()
            }
        }
    }

    fn make_move(&mut self, position: usize, player: &Player) {
        match player {
            Player::X => self.board[position - 1] = 1,
            Player::O => self.board[position - 1] = 2,
        };
    }
}
fn main() {
    let mut board = Board {
        board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    let mut result: Result;
    let mut turn: Player = Player::X;
    let mut turn_counter = 0;
    let mut player_choice = String::new();
    let player_turn: Player;
    loop {
        println!("Please pick either x or o");
        io::stdin().read_line(&mut player_choice).unwrap();
        player_choice = player_choice.trim().to_string();
        if player_choice == "x" || player_choice == "o" {
            break;
        } else {
            player_choice.clear();
        }
    }
    if player_choice == "x" {
        player_turn = Player::X;
    } else {
        player_turn = Player::O;
    }

    loop {
        if turn == player_turn {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            board.show_diagram();
            board.show_board();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input_enum = input.trim().parse::<i8>();
            let input_result: i8;
            match input_enum {
                Ok(i) => input_result = i,
                Err(_) => {
                    println!("error");
                    continue;
                }
            }
            if board.board[(input_result - 1) as usize] != 0 {
                println!("invalid move");
                continue;
            }

            turn_counter += 1;
            board.make_move(input_result as usize, &turn);
        } else {
            board.make_move(board.ai_move(&turn) as usize, &turn);
            board.show_diagram();
            board.show_board();
            turn_counter += 1
        }
        if turn_counter % 2 == 0 {
            turn = Player::X
        } else {
            turn = Player::O
        }
        result = board.is_a_win();
        match result {
            Result::Ongoing => continue,
            _ => break,
        }
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    match result {
        Result::Xwin => println!("X Won"),
        Result::Owin => println!("O Won"),
        Result::Draw => println!("Draw"),
        _ => panic!("should not have ended game"),
    };
    board.show_board();
}
