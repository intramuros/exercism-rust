#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank >= 0 && rank < 8, file >= 0 && file < 8) {
            (true, true) => Some(ChessPosition(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.0 == other.position.0
            || self.position.1 == other.position.1
            || (self.position.0 - other.position.0).abs()
                == (self.position.1 - other.position.1).abs()
        {
            return true;
        }
        false
    }
}
