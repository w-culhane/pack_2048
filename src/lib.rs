extern crate rand;
extern crate termion;

use rand::Rng;

type Tile = usize;
type Board = [[Tile; 4]; 4];

pub enum Move {
    North,
    West,
    East,
    South,
}

pub struct Game {
    pub board: Board,
    pub score: usize,
}

impl Game {
    pub fn gen_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let value = if rng.gen_weighted_bool(10) { 4 } else { 2 };
        // TODO: Make this only work for empty cells, instead of brute-forcing it
        loop {
            let tile_x = rng.gen_range(0, 4);
            let tile_y = rng.gen_range(0, 4);
            if self.board[tile_x][tile_y] == 0 {
                self.board[tile_x][tile_y] = value;
                break;
            }
        }
    }
    pub fn handle_move(board: &mut Board, game_move: Option<Move>) {
        match game_move {
            Some(Move::North) => {
                for column in 0..4 {
                    for row in (0..4).rev() {
                        for next_row in (row + 1)..4 {
                            if board[next_row][column] != 0 {
                                if board[row][column] == 0 as Tile {
                                    board[row][column] = board[next_row][column];
                                    board[next_row][column] = 0;
                                } else if board[row][column] == board[next_row][column] {
                                    board[row][column] += board[row][column];
                                    board[next_row][column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::West) => {
                for mut row in &mut board.iter_mut() {
                    for column in 0..3 {
                        for next_column in (column + 1)..4 {
                            if row[next_column] != 0 {
                                if row[column] == 0 as Tile {
                                    row[column] = row[next_column];
                                    row[next_column] = 0;
                                } else if row[column] == row[next_column] {
                                    row[column] += row[column];
                                    row[next_column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::East) => {
                for mut row in &mut board.iter_mut() {
                    for column in (0..4).rev() {
                        for next_column in (0..column).rev() {
                            if row[next_column] != 0 {
                                if row[column] == 0 as Tile {
                                    row[column] = row[next_column];
                                    row[next_column] = 0;
                                } else if row[column] == row[next_column] {
                                    row[column] += row[column];
                                    row[next_column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Some(Move::South) => {
                for column in 0..4 {
                    for row in (0..4).rev() {
                        for next_row in (0..row).rev() {
                            if board[next_row][column] != 0 {
                                if board[row][column] == 0 as Tile {
                                    board[row][column] = board[next_row][column];
                                    board[next_row][column] = 0;
                                } else if board[row][column] == board[next_row][column] {
                                    board[row][column] += board[row][column];
                                    board[next_row][column] = 0;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }
    pub fn get_text_board(&self) -> std::string::String {
        let divider: std::string::String = "+---+---+---+---+\r\n".to_string();
        let mut finished_string = String::new();
        for row in self.board.iter() {
            finished_string = finished_string + &divider;
            for tile in row.iter() {
                finished_string = finished_string + "| " + &tile.to_string() + " ";
            }
            finished_string = finished_string + "|\r\n"
        }
        finished_string = finished_string + &divider;
        return finished_string;
    }
    pub fn new() -> Game {
        let board = [[0; 4]; 4];
        Game {
            board: board,
            score: 0,
        }
    }
}
