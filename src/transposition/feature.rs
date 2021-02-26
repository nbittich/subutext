use std::collections::BTreeSet;

use super::cell::CellKey;
use super::{
    cell::{ACell, Cell},
    matrix::Matrix,
};

impl Matrix {
    pub fn slice_at(&mut self, cell: Box<dyn ACell>) -> Matrix {
        let right_slice: String = self
            .cells
            .split_off(&CellKey::from_cell_key(cell))
            .iter()
            .map(|c| c.value)
            .collect();
        Matrix::new(right_slice, self.max_nb_column)
    }
    pub fn shift_end(&mut self, cell: Box<dyn ACell>) -> &Matrix {
        let cells: BTreeSet<Cell> = self
            .cells
            .iter()
            .filter(|c| c.row == cell.get_row())
            .skip(cell.get_column())
            .map(|c| {
                let mut column = self.max_nb_column -1;
                if c.column != cell.get_column() {
                    column = c.get_column() - 1;
                }
                Cell {
                    row: c.row,
                    column: column,
                    value: c.value,
                }
            })
            .collect();

        cells.iter().for_each(|c| {
            self.cells.replace(*c);
        });

        self
    }
    pub fn merge_at(&mut self, start_at: CellKey, other_matrix: &mut Matrix) -> &Matrix {
        let last_row = self.cells.iter().last().map(|c| c.row).unwrap();
        other_matrix
            .cells
            .iter()
            .enumerate()
            .map(|(idx, c)| Cell {
                row: start_at.row,
                column: start_at.column + idx,
                value: c.value,
            })
            .for_each(|c| {
                let mut replaced = self.cells.replace(c);
                while let Some(to_replace) = replaced {
                    if to_replace.row < last_row {
                        replaced = self.cells.replace(Cell {
                            row: to_replace.row + 1,
                            column: to_replace.column,
                            value: to_replace.value,
                        });
                    } else {
                        replaced = self.cells.replace(Cell {
                            row: start_at.row,
                            column: to_replace.column + 1,
                            value: to_replace.value,
                        });
                    }
                }
            });

        self
    }
    pub fn merge(&mut self, other_matrix: &Matrix) -> Matrix {
        let mut text = self.to_string();
        let text_to_merge = other_matrix.to_string();
        text.push_str(&text_to_merge);
        Matrix::new(text, self.max_nb_column)
    }
    pub fn swap_letters(&mut self) -> &Self {
        let last_row = self.cells.iter().last().unwrap();
        let mut text = self.to_string().chars().collect::<Vec<char>>();
        let slices = text.chunks_mut(last_row.row + 1);
        self.max_nb_column = slices.len();
        self.cells = slices
            .enumerate()
            .flat_map(|(col, slice)| {
                slice.iter_mut().enumerate().map(move |(row, cell)| Cell {
                    row: row,
                    column: col,
                    value: *cell,
                })
            })
            .collect();
        self
    }
    pub fn diag(&self) -> Self {
        let last_row = self.cells.iter().last().unwrap();
        let max_col = self.max_nb_column;
        let mut text = self.to_string().chars().collect::<Vec<char>>();
        let new_text: String = text
            .chunks_mut(max_col)
            .map(|chars| chars.iter().collect::<String>())
            .enumerate()
            .map(|(i, slice)| {
                let advance_by = max_col / (last_row.row - 1) * i;
                slice[max_col - advance_by..].to_string() + &slice[..max_col - advance_by]
            })
            .collect();
        Matrix::new(new_text, max_col)
    }
}

#[cfg(test)]
mod test {
    use super::CellKey;
    use super::Matrix;

    #[test]
    fn slice_test() {
        let zod = std::fs::read_to_string("original.txt").unwrap();
        let mut matrix = Matrix::new(zod, 17);
        let mut second_matrix = matrix.slice_at(CellKey::new(8, 17));
        let mut third_matrix = second_matrix.slice_at(CellKey::new(8, 11));
        let fourth_matrix = third_matrix.slice_at(CellKey::new(0, 6));
        second_matrix.swap_letters();
        second_matrix.merge_at(CellKey::new_key(0, 11), &mut third_matrix);
        matrix.swap_letters();
        second_matrix = second_matrix.diag();
        second_matrix.shift_end(CellKey::new(5, 3));
        let cyp = std::fs::read_to_string("cypher.txt").unwrap();
        assert_eq!(cyp, matrix.diag().merge(&second_matrix).merge(&fourth_matrix).to_formatted_string());
    }
}
