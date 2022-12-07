use crate::{simple_parity_check::pbit_syndrome, Block};

pub fn count_error(datablocks: Vec<Block>) -> u32 {
    // Calculate the syndrome in integer form of each row
    // by bitXOR-ing the codewords.
    let column_syndrome_int = 0;
    datablocks
        .iter()
        .fold(column_syndrome_int, |column_syndrome_int, cword| {
            column_syndrome_int ^ cword.to_num()
        });

    // Collecting the syndromes of each column and row can also
    // detect if the parity bits in those areas are changed.
    // Calculate the sum for the next step.
    let column_syndrome = Block::from_num(column_syndrome_int)
        .peek()
        .iter()
        .map(|bit| *bit as u32)
        .sum::<u32>();

    let rows_syndrome = datablocks
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
