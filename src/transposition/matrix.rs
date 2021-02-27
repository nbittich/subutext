use std::collections::BTreeSet;

use super::cell::Cell;

pub struct Matrix {
    pub(super) max_nb_column: usize,
    pub(super) cells: BTreeSet<Cell>,
}

impl Matrix {
    pub fn new(text: String, max_columns: usize) -> Matrix {
        let only_alpha_numerics: Vec<char> = text.chars().filter(|c| c.is_alphanumeric()).collect();
        let text_len = only_alpha_numerics.len();
        let chunks = only_alpha_numerics.chunks(max_columns);
        let max_nb_column = if text_len > max_columns {
            max_columns
        } else {
            text_len
        };
        let cells: BTreeSet<Cell> = chunks
            .enumerate()
            .flat_map(|(row, chars)| {
                chars.iter().enumerate().map(move |(column, value)| Cell {
                    row,
                    column,
                    value: *value,
                })
            })
            .collect();

        Matrix {
            cells,
            max_nb_column,
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    #[test]
    fn iter_matrix() {
        let text = "this is a text avec du français";
        let matrix = Matrix::new(text.to_string(), 8);
        assert_eq!(25, matrix.len());
    }
}
