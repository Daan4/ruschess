use crate::game::Side;
use std::fmt;

pub struct Player {
    name: String,
    side: Side,
}

impl Player {
    pub fn new(name: String, side: Side) -> Player {
        Player{name, side}
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
