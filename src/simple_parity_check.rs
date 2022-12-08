use crate::{Bit, Block};

// Calculating syndrome and parity bit has the same algorithm
pub fn pbit_syndrome(bits: u32) -> Bit {
    (bits.count_ones() % 2 != 0) as Bit
}

pub fn encode(dataword: &Block) -> Block {
    let mut bits = dataword.to_num();
    let len = dataword.peek().len() + 1;
    // Generate a parity bit and append it.
    let parity_bit = pbit_syndrome(bits) as u32;
    bits = (bits << 1) + parity_bit;

    Block::from_num_with_size(bits, len)
}

pub fn decode(codeword: &Block) -> Option<Block> {
    if pbit_syndrome(codeword.to_num()) == 0 {
        let mut bits = codeword.to_num();
        let len = codeword.peek().len() - 1;
        // Get rid of the parity bit
        bits >>= 1;

        Some(Block::from_num_with_size(bits, len))
    } else {
        None
    }
}
