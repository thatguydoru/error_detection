use crate::{simple_parity_check::pbit_syndrome, Block};

pub fn count_error(data: Vec<Block>) -> u32 {
    // Initialize column vec
    let mut columns: Vec<Block> = vec![];

    for bit in data.first().unwrap().peek() {
        columns.push(Block::from_vec(vec![*bit]));
    }

    // Push rest of the bits to their respective columns
    for cword in data.iter().skip(1) {
        for (col, bit) in cword.peek().iter().enumerate() {
            columns[col].push_bit(*bit);
        }
    }

    // Collecting the syndromes of each column and row can also
    // detect if the parity bits in those areas are changed.
    // Calculate the sum for the next step.
    let column_syndrome = columns
        .into_iter()
        .map(|cword| pbit_syndrome(cword.to_num()) as u32)
        .sum::<u32>();

    let rows_syndrome = data
        .into_iter()
        .map(|cword| pbit_syndrome(cword.to_num()) as u32)
        .sum::<u32>();

    // This part is due to some of my observations:
    // - Rows can detect errors if they overlap with respect to the column,
    //   column can't if the number of overlapping errors are even.
    // - The same goes for column when errors overlap with respect to the rows.
    // - If errors are spread out, such that there are no overlapping indexes,
    //   then all those errors are detected. If columns > rows, then the columns
    //   will reflect the number of errors detected. The same goes vice versa.
    rows_syndrome.max(column_syndrome)
}
