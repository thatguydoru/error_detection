pub mod simple_parity_check;
pub mod two_dim_parity_check;
pub mod checksum;

pub type Bit = u8;

// Dataword and Codeword are Block of data
#[derive(Debug, PartialEq, Eq)]
pub struct Block(Vec<Bit>);

impl Block {
    pub fn new(block: &str) -> Self {
        let dword = block
            .chars()
            .map(|char| char.to_digit(2).expect("Must be binary") as Bit)
            .collect();

        Self(dword)
    }

    pub fn peek(&self) -> &Vec<Bit> {
        &self.0
    }
}

pub fn parse(data: &str) -> Vec<Block> {
    // Data looks like this:
    // 101100111 101010111 010110100 110101011 100101111
    // Need to turn it to this:
    // vec[Block(101100111), Block(101010111), ..., Block(100101111)]
    data.split_whitespace().map(Block::new).collect()
}
