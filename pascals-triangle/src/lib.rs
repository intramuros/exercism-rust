pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vals: Vec<Vec<u32>> = Vec::with_capacity((1..(self.row_count + 1) as usize).sum());
        for i in 0..(self.row_count) as usize {
            let row: Vec<u32> = vec![1; i + 1]
                .iter()
                .enumerate()
                .map(|(j, &x)| {
                    if j == 0 || j == i {
                        x
                    } else {
                        vals[i - 1][j - 1] + vals[i - 1][j]
                    }
                }).collect();
            vals.push(row);
        }
        vals
    }
}
