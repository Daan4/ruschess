use std::fmt;

pub struct Position {
    r#file: usize,
    rank: usize
}

impl Position {
    pub fn new(r#file: usize, rank: usize) -> Position {
        Position{r#file, rank}
    }

    fn valid(&self) -> bool {
        self.r#file > 0 && self.r#file < 9 && self.rank > 0 && self.rank < 9
    }
}

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.valid() {
            write!(f, "{}{}", FILES[self.file-1], self.rank)
        } else {
            write!(f, "Invalid Position")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn position() {
        let pos = Position::new(1, 1);
        assert_eq!(pos.valid(), true);
        let pos = Position::new(8, 8);
        assert_eq!(pos.valid(), true);
        let pos = Position::new(0, 8);
        assert_eq!(pos.valid(), false);
        let pos = Position::new(1, 9);
        assert_eq!(pos.valid(), false);
    }
}
