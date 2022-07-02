use crate::position::Position;

pub trait Piece {
    fn r#move(&mut self, destination: Position) -> Result<(), String>;
}

pub struct King {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl King {
    pub fn new(position: Position) -> King {
        King{position, captured: false, may_castle: true}
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
}

pub struct Queen {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl Queen {
    pub fn new(position: Position) -> Queen {
        Queen{position, captured: false, may_castle: false}
    }
}

impl Piece for Queen {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }
}

pub struct Rook {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl Rook {
    pub fn new(position: Position) -> Rook {
        Rook{position, captured: false, may_castle: true}
    }
}

impl Piece for Rook {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }
}

pub struct Bishop {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl Bishop {
    pub fn new(position: Position) -> Bishop {
        Bishop{position, captured: false, may_castle: false}
    }
}

impl Piece for Bishop {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }
}

pub struct Knight {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl Knight {
    pub fn new(position: Position) -> Knight {
        Knight{position, captured: false, may_castle: false}
    }
}

impl Piece for Knight {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }
}

pub struct Pawn {
    position: Position,
    captured: bool,
    may_castle: bool
}

impl Pawn {
    pub fn new(position: Position) -> Pawn {
        Pawn{position, captured: false, may_castle: false}
    }
}

impl Piece for Pawn {
    fn r#move(&mut self, destination: Position) -> Result<(), String> {
        Ok(())
    }
}
