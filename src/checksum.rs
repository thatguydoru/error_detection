use crate::{Bit, Block};

fn add_bits(a: Bit, b: Bit) -> (Bit, Bit) {
    let sum = a ^ b;
    let carry = a & b;

    (sum, carry)
}

fn add_blocks(a_block: &Block, b_block: &Block) -> (u32, u32) {
    let a = a_block.to_num();
    let b = b_block.to_num();


    todo!()
}

#[cfg(test)]
mod helper_tests {
    use crate::{checksum::add_blocks, Block};

    #[test]
    fn add_blocks_test() {
        let a = Block::from_string("11");
        let b = Block::from_string("01");

        assert_eq!((0b0_u32, 0b1_u32), add_blocks(&a, &b));
    }
}
