pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    use Direction::*;
    let mut res = vec![vec![0; size as usize]; size as usize];
    let mut row = 0;
    let mut col = 0;
    let mut dir = Right;
    for s in 1..=size * size {
        res[row as usize][col as usize] = s;
        let (dx, dy) = dir.dxdy();
        match dir {
            Right => {
                if col + dy == size as i8 || res[row as usize][(col + dy) as usize] != 0 {
                    dir = Down;
                }
            }
            Left => {
                if col + dy == -1 || res[row as usize][(col + dy) as usize] != 0 {
                    dir = Up;
                }
            }
            Up => {
                if row + dx == -1 || res[(row + dx) as usize][col as usize] != 0 {
                    dir = Right;
                }
            }
            Down => {
                if row + dx == size as i8 || res[(row + dx) as usize][col as usize] != 0 {
                    dir = Left;
                }
            }
        };

        let (dx, dy) = dir.dxdy();
        row += dx;
        col += dy;
    }
    res
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn dxdy(&self) -> (i8, i8) {
        match self {
            Self::Right => (0, 1),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
    }
}
