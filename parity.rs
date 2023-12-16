fn calc_row_parity(row: &[i8]) -> i8 {
    row.iter().fold(0, |parity, &bit| parity ^ bit)
}

fn calc_column_parity(matrix: &[Vec<i8>], index: usize) -> i8 {
    matrix.iter().fold(0, |parity, row| parity ^ row[index])
}

fn main() {
    let mut matrix: Vec<Vec<i8>> = vec![
        vec![1, 1, 1, 0],
        vec![0, 1, 1, 0],
        vec![1, 0, 0, 1],
        vec![1, 1, 0, 1],
    ];

    for row in &mut matrix {
        let row_parity = calc_row_parity(row);
        row.push(row_parity);
    }

    let column_parities: Vec<i8> = (0..matrix[0].len())
        .map(|i| calc_column_parity(&matrix, i))
        .collect();

    matrix.push(column_parities);
    matrix.iter().for_each(|r| println!("{:?}", r));
}
