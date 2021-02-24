use super::cell::CellKey;
use super::{
    cell::{ACell, Cell},
    matrix::Matrix,
};

impl Matrix {
    fn slice_at(&mut self, cell: Box<dyn ACell>) -> Matrix {
        let right_slice = self.cells.split_off(&CellKey::from_cell_key(cell));
        Matrix {
            cells: right_slice,
            max_nb_column: self.max_nb_column,
        }
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
    fn diag(&self) -> Self {
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
        let text = "this is a text avec du français";
        let mut matrix = Matrix::new(text.to_string(), 8);
        let matrix_slice = matrix.slice_at(CellKey::new(1, 4));
        assert_eq!(12, matrix.len());
        assert_eq!(13, matrix_slice.len());
        assert_eq!("thisisatexta", matrix.to_string());
        assert_eq!("vecdufrançais", matrix_slice.to_string());
        let mut matrix = Matrix::new(text.to_string(), 8);
        matrix.swap_letters();
        assert_eq!("tievunshsxefçiatcrastadai", matrix.to_string());

        let zod = std::fs::read_to_string("original.txt").unwrap();
        let mut matrix = Matrix::new(zod, 17);
        matrix.slice_at(CellKey::new(8, 17));
        println!("{:?}", matrix.swap_letters());
        println!("========");
        println!("{:?}", matrix.diag());
    }
}
