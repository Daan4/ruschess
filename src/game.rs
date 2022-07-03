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
        let white = Player::new(white_name, Side::White);
        let mut pieces: Vec<Box<dyn Piece>> = vec![];

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
        let mut pieces: Vec<Box<dyn Piece>> = vec![];

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
                    return Err(format!("Invalid move: {:?} to move", self.turn))
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
        write!(f, "placeholder")
    }
}
