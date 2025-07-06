#[derive(Debug)]
pub struct Matrix {
    values: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let v: Vec<Vec<u32>> = Matrix::parse_input(input);
        Matrix { values: v }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        match row_no {
            row_no if row_no > 0 => self.get_row(row_no - 1),
            _ => None,
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        match col_no {
            col_no if col_no > 0 => self.get_column(col_no - 1),
            _ => None,
        }
    }

    fn get_column(&self, idx: usize) -> Option<Vec<u32>> {
        let values: Vec<u32> = self
            .values
            .iter()
            .filter_map(|row| row.get(idx).copied())
            .collect();
        match values {
            values if values.is_empty() => None,
            _ => Some(values),
        }
    }

    fn get_row(&self, idx: usize) -> Option<Vec<u32>> {
        self.values.get(idx).cloned()
    }

    pub fn parse_input(s: &str) -> Vec<Vec<u32>> {
        s.lines().map(Matrix::line_to_vec).collect()
    }
    // TODO: make it private
    pub fn line_to_vec(line: &str) -> Vec<u32> {
        line.trim()
            .split(" ")
            .map(|token| token.parse::<u32>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_of_1() {
        let expected = vec![1];
        assert_eq!(expected, Matrix::line_to_vec("1"));
    }

    #[test]
    fn parse_line_of_1_2_3() {
        let expected = vec![1, 2, 3];
        assert_eq!(expected, Matrix::line_to_vec("1 2 3"));
    }

    #[test]
    fn parse_input_of_1() {
        let expected = vec![vec![1]];
        let values = Matrix::parse_input("1");
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_input_of_1_2_3() {
        let expected = vec![vec![1, 2, 3]];
        let values = Matrix::parse_input("1 2 3");
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_input_of_1_2_and_3_4() {
        let expected = vec![vec![1, 2], vec![3, 4]];
        let values = Matrix::parse_input("1 2 \n 3 4");
        assert_eq!(expected, values);
    }

    #[test]
    // #[ignore]
    fn construct_with_empty_string() {
        let expected = Matrix { values: vec![] };
        let matrix = Matrix::new("");
        assert_eq!(expected.values, matrix.values);
    }

    #[test]
    // #[ignore]
    fn construct_with_one_string() {
        let expected = Matrix {
            values: vec![vec![1]],
        };
        let matrix = Matrix::new("1");
        assert_eq!(expected.values, matrix.values);
    }
}
