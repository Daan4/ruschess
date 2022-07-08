use crate::position::Position;
use crate::game::Side;
use std::fmt;

pub trait Piece: fmt::Display {
    // Move a piece, returns Err if the destination is invalid
    fn r#move(&mut self, destination: Position) -> Result<(), String>;

    // Check if a piece sees a position, returns Err if the position is invalid
    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String>;

    fn position(&self) -> Position;

    fn side(&self) -> Side;

    fn captured(&self) -> bool;
}

pub struct King {
    position: Position,
    captured: bool,
    may_castle: bool,
    side: Side,
}

impl King {
    pub fn new(position: Position, side: Side) -> King {
        King{position, captured: false, may_castle: true, side}
    }
}

impl Piece for King {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        // todo implement
        if !destination.valid() {
            return Err(destination.to_string())
        } else if self.position.distance(destination) > 1 {
            return Err("Invalid move".to_owned())
        } else {
            self.position = destination;
            return Ok(())
        }
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "K"),
            Side::Black => write!(f, "k"),
        }        
    }
}

pub struct Queen {
    position: Position,
    captured: bool,
    side: Side,
}

impl Queen {
    pub fn new(position: Position, side:Side) -> Queen {
        Queen{position, captured: false, side}
    }
}

impl Piece for Queen {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "Q"),
            Side::Black => write!(f, "q"),
        }        
    }
}

pub struct Rook {
    position: Position,
    captured: bool,
    may_castle: bool,
    side: Side,
}

impl Rook {
    pub fn new(position: Position, side: Side) -> Rook {
        Rook{position, captured: false, may_castle: true, side}
    }
}

impl Piece for Rook {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "R"),
            Side::Black => write!(f, "r"),
        }        
    }
}

pub struct Bishop {
    position: Position,
    captured: bool,
    side: Side,
}

impl Bishop {
    pub fn new(position: Position, side: Side) -> Bishop {
        Bishop{position, captured: false, side}
    }
}

impl Piece for Bishop {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "B"),
            Side::Black => write!(f, "b"),
        }        
    }
}

pub struct Knight {
    position: Position,
    captured: bool,
    side: Side,
}

impl Knight {
    pub fn new(position: Position, side: Side) -> Knight {
        Knight{position, captured: false, side}
    }
}

impl Piece for Knight {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "N"),
            Side::Black => write!(f, "n"),
        }        
    }
}

pub struct Pawn {
    position: Position,
    captured: bool,
    has_moved: bool,
    side: Side,
}

impl Pawn {
    pub fn new(position: Position, side: Side) -> Pawn {
        Pawn{position, captured: false, has_moved: false, side}
    }
}

impl Piece for Pawn {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }

    fn sees(&self, position: Position, pieces: Vec<Box<dyn Piece>>) -> Result<(), String> {
        Ok(())
    }

    fn position(&self) -> Position {
        self.position
    }

    fn side(&self) -> Side {
        self.side
    }

    fn captured(&self) -> bool {
        self.captured
    }
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.side() {
            Side::White => write!(f, "P"),
            Side::Black => write!(f, "p"),
        }        
    }
}
