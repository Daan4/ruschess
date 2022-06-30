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

    fn distance(&self, position: Position) -> usize {
        self.r#file.abs_diff(position.r#file) + self.rank.abs_diff(position.rank)
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
    fn position_valid() {
        let pos = Position::new(1, 1);
        assert_eq!(pos.valid(), true);

        let pos = Position::new(8, 8);
        assert_eq!(pos.valid(), true);

        let pos = Position::new(0, 8);
        assert_eq!(pos.valid(), false);

        let pos = Position::new(1, 9);
        assert_eq!(pos.valid(), false);
    }

    #[test]
    fn position_distance() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 0);
        assert_eq!(pos1.distance(pos2), 0);

        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(1, 0);
        assert_eq!(pos1.distance(pos2), 1);

        let pos1 = Position::new(1, 0);
        let pos2 = Position::new(0, 0);
        assert_eq!(pos1.distance(pos2), 1);

        let pos1 = Position::new(0, 1);
        let pos2 = Position::new(0, 0);
        assert_eq!(pos1.distance(pos2), 1);

        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(0, 1);
        assert_eq!(pos1.distance(pos2), 1);

        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(4, 8);
        assert_eq!(pos1.distance(pos2), 9);
    }
}
