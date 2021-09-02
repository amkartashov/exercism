pub fn annotate(minefield: &[&str]) -> Vec<String> {
    Minefield::new(minefield).to_strings()
}

struct Minefield(Vec<Vec<Minecell>>);

impl Minefield {
    fn to_strings(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|r| r.iter().map(Minecell::to_char).collect())
            .collect()
    }

    fn new(minefield: &[&str]) -> Self {
        Minefield(
            minefield
                .iter()
                .map(|&r| r.chars().map(Minecell::new).collect())
                .collect(),
        )
        .update_counters()
    }

    fn update_counters(mut self) -> Self {
        let (rows, cols) = (self.rows(), self.cols());
        for r in 0..rows {
            for c in 0..cols {
                if self.0[r][c] == Minecell::Mine {
                    for (i, j) in Minefield::neighbours(r, c, rows, cols) {
                        match self.0[i][j] {
                            Minecell::Mine => {}
                            Minecell::Counter(x) => self.0[i][j] = Minecell::Counter(x + 1),
                        }
                    }
                }
            }
        }
        self
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        if self.rows() > 0 {
            self.0[0].len()
        } else {
            0
        }
    }

    /// Returns iterator over neighbour cells coordinates
    fn neighbours(
        row: usize,
        col: usize,
        rows: usize,
        cols: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        // rows range
        (row.saturating_sub(1)..=std::cmp::min(row + 1, rows.saturating_sub(1)))
            .flat_map(move |r|
            // columns range
            (col.saturating_sub(1)..=std::cmp::min(col + 1, cols.saturating_sub(1)))
                .map(move |c| (r, c)))
            .filter(move |&(r, c)| !(r == row && c == col))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Minecell {
    Mine,
    Counter(u8),
}

impl Minecell {
    pub fn new(c: char) -> Self {
        match c {
            '*' => Minecell::Mine,
            _ => Minecell::Counter(0),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            &Minecell::Mine => '*',
            &Minecell::Counter(0) => ' ',
            &Minecell::Counter(x) if x < 10 => ('0' as u8 + x) as char,
            _ => unreachable!("x shouln't be bigger than 8"),
        }
    }
}
