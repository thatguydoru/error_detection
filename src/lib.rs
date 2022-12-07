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
