pub struct PascalsTriangle{
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self{row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::with_capacity(self.row_count as usize);
        
        for i in 1..=(self.row_count as usize) {
            match i {
                1 => rows.push(vec![1]),
                2 => rows.push(vec![1,1]),
                _ => {
                    let mut row = Vec::with_capacity(i);
                    row.push(1);
                    row.extend(rows[i - 2].windows(2).map(|a| a[0] + a[1]));
                    row.push(1);
                    rows.push(row);
                },
            }
        }

        rows
    }
}
