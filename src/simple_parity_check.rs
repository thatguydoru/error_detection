use crate::{Bit, Block};

pub fn encode(dataword: &Block) -> Block {
    let mut bits = dataword.peek().clone();
    // Generate a parity bit and append it.
    bits.push(pbit_syndrome(dataword.peek()));

    Block(bits)
}

// Calculating syndrome and parity bit has the same algorithm
pub fn pbit_syndrome(block: &[Bit]) -> Bit {
    //(block.iter().sum::<u8>() % 2 != 0) as Bit
    let mut result = 0;
    for bit in block {
        result ^= bit;
    }

    result
}

pub fn decode(codeword: &Block) -> Option<Block> {
    if pbit_syndrome(codeword.peek()) == 0 {
        let mut bits = codeword.peek().clone();
        // Get rid of the parity bit
        bits.pop();

        Some(Block(bits))
    } else {
        None
    }
}
