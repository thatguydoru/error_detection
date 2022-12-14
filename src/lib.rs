pub mod checksum;
pub mod crc;
pub mod simple_parity_check;
pub mod two_dim_parity_check;

use std::fmt::{self, Display};

pub type Bit = u8;

// Dataword and Codeword are Block of data
#[derive(Debug, PartialEq, Eq)]
pub struct Block(Vec<Bit>);

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = self.0.iter().map(|bit| bit.to_string()).collect::<String>();
        write!(f, "{}", block)
    }
}

impl Block {
    pub fn from_string(block: &str) -> Self {
        let dword = block
            .chars()
            .map(|char| char.to_digit(2).expect("Must be binary") as Bit)
            .collect();

        Self(dword)
    }

    pub fn from_vec(bits: Vec<Bit>) -> Self {
        Self(bits)
    }

    pub fn from_num(num: u32) -> Self {
        let mut quotient = num;
        let mut bits = vec![];

        while quotient > 0 {
            bits.push((quotient % 2) as Bit);
            quotient /= 2;
        }
        bits.reverse();

        Self(bits)
    }

    pub fn from_num_with_size(num: u32, size: usize) -> Self {
        let mut quotient = num;
        let mut bits = vec![0; size];

        let mut index = 0;
        while quotient > 0 {
            bits[index] = (quotient % 2) as Bit;
            quotient /= 2;
            index += 1;
        }
        bits.reverse();

        Self(bits)
    }

    pub fn peek(&self) -> &Vec<Bit> {
        &self.0
    }

    pub fn push_bit(&mut self, bit: Bit) {
        self.0.push(bit);
    }

    pub fn to_num(&self) -> u32 {
        let mut multiplier = 1;
        let mut sum = 0;

        for bit in self.0.iter().rev() {
            sum += *bit as u32 * multiplier;
            multiplier *= 2;
        }

        sum
    }
}

pub fn parse(stream: &str) -> Vec<Block> {
    // Stream looks like this:
    // 101100111 101010111 010110100 110101011 100101111
    // Need to turn it to this:
    // vec[Block(101100111), Block(101010111), ..., Block(100101111)]
    stream.split_whitespace().map(Block::from_string).collect()
}
