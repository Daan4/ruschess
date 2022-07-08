use crate::player::Player;
use crate::position::Position;
use crate::piece::*;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    White,
    Black,
}

pub struct Game {
    pub white: Player,
    pub black: Player,
    turn: Side,
    pieces: Vec<Box<dyn Piece>>,
}

impl Game {
    pub fn new(white_name: String, black_name: String) -> Game {
        let mut pieces: Vec<Box<dyn Piece>> = vec![];

        let white = Player::new(white_name, Side::White);   
        for file in 1..9 {
            pieces.push(Box::new(Pawn::new(Position::new(file, 2), Side::White)));
        }        
        pieces.push(Box::new(Rook::new(Position::new(1, 1), Side::White)));
        pieces.push(Box::new(Rook::new(Position::new(8, 1), Side::White)));
        pieces.push(Box::new(Knight::new(Position::new(2, 1), Side::White)));
        pieces.push(Box::new(Knight::new(Position::new(7, 1), Side::White)));
        pieces.push(Box::new(Bishop::new(Position::new(3, 1), Side::White)));
        pieces.push(Box::new(Bishop::new(Position::new(6, 1), Side::White)));
        pieces.push(Box::new(Queen::new(Position::new(4, 1), Side::White)));
        pieces.push(Box::new(King::new(Position::new(5, 1), Side::White)));

        let black = Player::new(black_name, Side::Black);
        for file in 1..9 {
            pieces.push(Box::new(Pawn::new(Position::new(file, 7), Side::Black)));
        }        
        pieces.push(Box::new(Rook::new(Position::new(1, 8), Side::Black)));
        pieces.push(Box::new(Rook::new(Position::new(8, 8), Side::Black)));
        pieces.push(Box::new(Knight::new(Position::new(2, 8), Side::Black)));
        pieces.push(Box::new(Knight::new(Position::new(7, 8), Side::Black)));
        pieces.push(Box::new(Bishop::new(Position::new(3, 8), Side::Black)));
        pieces.push(Box::new(Bishop::new(Position::new(6, 8), Side::Black)));
        pieces.push(Box::new(Queen::new(Position::new(4, 8), Side::Black)));
        pieces.push(Box::new(King::new(Position::new(5, 8), Side::Black)));

        Game{white, black, turn: Side::White, pieces}
    }

    pub fn r#move(&mut self, position: Position, destination: Position) -> Result<(), String> {
        if !position.valid() {
            return Err("Invalid Position".to_owned())
        }
        if !destination.valid() {
            return Err("Invalid Destination".to_owned())
        }
        
        let turn_side = self.turn;
        match self.get_piece(position) {
            Some(piece) => {
                if piece.side() != turn_side {
                    return Err(format!("Invalid move: Move a {:?} piece", self.turn))
                }

                piece.r#move(destination)
            },
            None => Err(format!("No piece on {}", position))
        }
    }

    fn get_piece(&mut self, position: Position) -> Option<&mut Box<dyn Piece>> {
        for piece in &mut self.pieces {
            if piece.position() == position {
                return Some(piece) 
            }
        }
        None
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board: [[Option<&dyn Piece>; 8]; 8] = [[None; 8]; 8];

        for piece in &self.pieces {
            if !piece.captured() {
                let rank = piece.position().rank();
                let file = piece.position().file();
                board[rank-1][file-1] = Some(piece.as_ref());
            }
        }

        let mut buf: String = " ________\n".to_string();
        for rank in 0..8 {
            let mut line: String = (8-rank).to_string();
            for file in 0..8 {
                match board[7-rank][file] {
                    Some(piece) => line = line + &piece.to_string(),
                    None => line = line + " "
                }
            }
            line = line + "|";
            if rank == 0 {
                line = format!("{} {}", line, &self.black.to_string());
                if self.turn == Side::Black {
                    line = line + " X";
                }
            } else if rank == 7 {
                line = format!("{} {}", line, &self.white.to_string());
                if self.turn == Side::White {
                    line = line + " X";
                }
            }
            line = line + "\n";
            buf = buf + &line;
        }
        buf = buf + " ABCDEFGH";

        write!(f, "{}", buf)
    }
}
