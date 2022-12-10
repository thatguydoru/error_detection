use crate::Block;

const KEYWORD: &str = "1011";
const CRC_BIT_LEN: usize = 3;

// Reference: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
pub fn crc_check<'a>(data: &Block) -> &'a str {
    let keyword_block = Block::from_string(KEYWORD);
    let keyword_len = keyword_block.peek().len();
    let input_len = data.peek().len() - CRC_BIT_LEN;
    let mut data = data.peek().clone();

    while data[..input_len].contains(&1) {
        let cur_shift = data.iter().position(|bit| *bit == 1_u8).unwrap();

        for i in 0..keyword_len {
            data[cur_shift + i] ^= keyword_block.peek()[i];
        }
    }

    if !data.contains(&1) {
        "Accept data"
    } else {
        "CRC error detected"
    }
}
