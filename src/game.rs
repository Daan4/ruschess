use crate::player::Player;
use crate::position::Position;
use crate::piece::*;

#[derive(Debug)]
pub enum Side {
    White,
    Black,
}

pub struct Game {
    pub white: Player,
    pub black: Player,
    turn: Side,
    white_pieces: Vec<Box<dyn Piece>>,
    black_pieces: Vec<Box<dyn Piece>>,
}

impl Game {
    pub fn new(white_name: String, black_name: String) -> Game {
        let white = Player::new(white_name, Side::White);
        let mut white_pieces: Vec<Box<dyn Piece>> = vec![];

        for file in 1..9 {
            white_pieces.push(Box::new(Pawn::new(Position::new(file, 2))));
        }        
        white_pieces.push(Box::new(Rook::new(Position::new(1, 1))));
        white_pieces.push(Box::new(Rook::new(Position::new(8, 1))));
        white_pieces.push(Box::new(Knight::new(Position::new(2, 1))));
        white_pieces.push(Box::new(Knight::new(Position::new(7, 1))));
        white_pieces.push(Box::new(Bishop::new(Position::new(3, 1))));
        white_pieces.push(Box::new(Bishop::new(Position::new(6, 1))));
        white_pieces.push(Box::new(Queen::new(Position::new(4, 1))));
        white_pieces.push(Box::new(King::new(Position::new(5, 1))));

        let black = Player::new(black_name, Side::Black);
        let mut black_pieces: Vec<Box<dyn Piece>> = vec![];

        for file in 1..9 {
            black_pieces.push(Box::new(Pawn::new(Position::new(file, 7))));
        }        
        black_pieces.push(Box::new(Rook::new(Position::new(1, 8))));
        black_pieces.push(Box::new(Rook::new(Position::new(8, 8))));
        black_pieces.push(Box::new(Knight::new(Position::new(2, 8))));
        black_pieces.push(Box::new(Knight::new(Position::new(7, 8))));
        black_pieces.push(Box::new(Bishop::new(Position::new(3, 8))));
        black_pieces.push(Box::new(Bishop::new(Position::new(6, 8))));
        black_pieces.push(Box::new(Queen::new(Position::new(4, 8))));
        black_pieces.push(Box::new(King::new(Position::new(5, 8))));

        Game{white, black, turn: Side::White, white_pieces, black_pieces}
    }

    pub fn r#move(&self, position: Position, destination: Position) {
        
    }
}
