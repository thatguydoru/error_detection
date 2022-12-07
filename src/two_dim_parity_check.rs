use crate::{simple_parity_check::pbit_syndrome, Bit, Block};

pub fn parse(data: &str) -> Vec<Block> {
    // Data looks like this:
    // 101100111 101010111 010110100 110101011 100101111
    // Need to turn it to this:
    // vec[Codeword(101100111), Codeword(101010111), ..., Codeword(100101111)]
    data.split_whitespace().map(Block::new).collect()
}

pub fn count_error(data: Vec<Block>) -> u32 {
    // Initialize column vec
    let mut columns: Vec<Vec<Bit>> = vec![];
    for bit in data.first().unwrap().peek() {
        columns.push(vec![*bit]);
    }

    // Skip first codeword as it is in the vec already,
    // then push bit to its respective column
    for cword in data.iter().skip(1) {
        for (col, bit) in cword.peek().iter().enumerate() {
            columns[col].push(*bit);
        }
    }

    // Collecting the syndromes of each column and row can also
    // detect if the parity bits in those areas are changed.
    // Calculate the sum for the next step.
    let column_syndrome = columns
        .into_iter()
        .map(|cword| pbit_syndrome(&cword) as u32)
        .sum::<u32>();

    let rows_syndrome = data
        .into_iter()
        .map(|cword| pbit_syndrome(cword.peek()) as u32)
        .sum::<u32>();

    // This part is due to some of my observations:
    // - Rows can detect errors if they overlap with respect to the column,
    //   column can't if the number of overlapping errors are even.
    // - The same goes for column when errors overlap with respect to the rows.
    // - If errors are spread out, such that there are no overlapping indexes,
    //   then all those errors are detected.
    // TODO: Proof?
    rows_syndrome.max(column_syndrome)
}
