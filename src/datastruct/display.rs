use std::fmt::{Debug, Display, Formatter, Result};

use super::matrix::Matrix;

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let max_column = self.max_nb_column;
        let mut current_row: i32 = -1;
        if max_column > 0 {
            write!(f, "|*|\t")?;
            for i in 0..max_column {
                write!(f, "|{}|\t", i)?;
            }
            for cell in &self.cells {
                let row = cell.row as i32;
                if current_row != row {
                    current_row = row;
                    writeln!(f, "")?;
                    write!(f, "|{}|\t", cell.row)?;
                }
                write!(f, "|{}|\t", cell.value)?;
            }
        }
        write!(f, "")
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s: String = self.cells.iter().map(|cell| cell.value).collect();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    #[test]
    fn iter_matrix() {
        let text = "this is a text avec du français";
        let matrix = Matrix::new(text.to_string(), 8);
        assert_eq!("thisisatextavecdufrançais", matrix.to_string());
    }
}
