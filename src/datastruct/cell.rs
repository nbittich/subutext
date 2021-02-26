use std::cmp::Ordering;
// this whole mess is totally useless, or does it?

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub(super) row: usize,
    pub(super) column: usize,
    pub(super) value: char,
}

pub trait ACell {
    fn get_row(&self) -> usize;
    fn get_column(&self) -> usize;
    fn get_value(&self) -> char;
}

pub trait Compare: ACell {
    fn compare(&self, other: &Self) -> Option<Ordering> {
        match self.get_row().cmp(&other.get_row()) {
            Ordering::Equal => Some(self.get_column().cmp(&other.get_column())),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}

impl ACell for Cell {
    fn get_row(&self) -> usize {
        self.row
    }

    fn get_column(&self) -> usize {
        self.column
    }

    fn get_value(&self) -> char {
        self.value
    }
}

pub struct CellKey {
    pub row: usize,
    pub column: usize,
}

impl ACell for CellKey {
    fn get_row(&self) -> usize {
        self.row
    }

    fn get_column(&self) -> usize {
        self.column
    }

    fn get_value(&self) -> char {
        'x'
    }
}

impl CellKey {
    pub fn new(row: usize, column: usize) -> Box<dyn ACell> {
        Box::new(CellKey { row, column })
    }
    pub fn new_key(row: usize, column: usize) -> CellKey {
        CellKey { row, column }
    }
    pub(super) fn from_cell_key(cell: Box<dyn ACell>) -> Cell {
        //todo this sucks a lot...
        Cell {
            row: cell.get_row(),
            column: cell.get_column(),
            value: 'x',
        }
    }
    pub(super) fn as_cell(row: usize, col: usize) -> Cell {
        Self::from_cell_key(Self::new(row, col))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.partial_cmp(other) == Some(Ordering::Equal);
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Cell {}
impl Compare for Cell {}
impl Compare for CellKey {}
