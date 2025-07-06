#[derive(Debug)]
pub struct Matrix {
    values: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let v: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .split(" ")
                    .map(|token| token.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        Matrix { values: v }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        match self.values.get(row_no - 1) {
            Some(val) => Some(val).cloned(),
            None => None,
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let values: Vec<u32> = self
            .values
            .iter()
            .filter_map(|row| row.get(col_no - 1).copied())
            .collect();
        match values {
            values if values.is_empty() => None,
            _ => Some(values),
        }
    }
}
