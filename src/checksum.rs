use crate::Block;

fn ones_complement_add(a: &Block, b: &Block) -> Block {
    let a = a.to_num() as u8;
    let b = b.to_num() as u8;

    let (mut sum, mut carry) = a.overflowing_add(b);

    while carry {
        (sum, carry) = sum.overflowing_add(carry as u8);
    }

    Block::from_num_with_size(sum as u32, 8)
}

fn complement(block: &Block) -> Block {
    let complement = block
        .peek()
        .iter()
        .map(|bit| u8::from(*bit == 0))
        .collect();

    Block::from_vec(complement)
}

pub fn checksum_decision<'a>(stream: &[Block]) -> &'a str {
    let mut sum = Block::from_string("0");

    for block in stream {
        sum = ones_complement_add(&sum, block);
    }

    let complement = complement(&sum);

    if complement.to_num() == 0 {
        "Accept data"
    } else {
        "Checksum error detected"
    }
}

#[cfg(test)]
mod helper_tests {
    use crate::{checksum::ones_complement_add, Block};

    #[test]
    fn ones_complement_test() {
        let a = Block::from_string("10011001");
        let b = Block::from_string("11100010");

        assert_eq!(ones_complement_add(&a, &b), Block::from_string("01111100"));
    }
}
