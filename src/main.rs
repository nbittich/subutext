use zodiac_cypher::{homophonic::transformer::HomophonCypher, inputhandler::file, transposition::{cell::CellKey, matrix::Matrix}};

fn main() {
    let zod = file::read_file(
        ["".to_string(), "original.txt".to_string()]
            .iter()
            .map(|s| s.to_string()),
    );
    let mut matrix = Matrix::new(zod.unwrap(), 17);
    let mut second_matrix = matrix.slice_at(CellKey::new(8, 17));
    let mut third_matrix = second_matrix.slice_at(CellKey::new(8, 11));
    let fourth_matrix = third_matrix.slice_at(CellKey::new(0, 6));
    second_matrix.swap_letters();
    second_matrix.merge_at(CellKey::new_key(0, 11), &mut third_matrix);
    matrix.swap_letters();
    second_matrix = second_matrix.diag();
    second_matrix.shift_end(CellKey::new(5, 3));
    let transposition = matrix.diag().merge(&second_matrix).merge(&fourth_matrix).to_formatted_string();
    let cypher = HomophonCypher::new(transposition);
    println!("{}", cypher.transform())
}
