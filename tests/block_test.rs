use error_detection::{parse, Block};

#[test]
fn dword_to_vec_bytes() {
    let dword_str = "1010";
    let cmp = vec![1, 0, 1, 0];

    assert!(cmp.eq(Block::from_string(dword_str).peek()));
}

#[test]
fn cword_to_vec_bytes() {
    let cword_str = "1010";
    let cmp = vec![1, 0, 1, 0];

    assert!(cmp.eq(Block::from_string(cword_str).peek()));
}

#[test]
fn from_num_test() {
    let block = Block::from_string("11111110");
    let len = block.peek().len();

    assert_eq!(Block::from_num_with_size(254, len), block);
}

#[test]
fn to_num_test() {
    let block = Block::from_string("11111111");

    assert_eq!(255, block.to_num());
}

#[test]
fn parse_success() {
    let data = "101100111 101010111 010110100 110101011 100101111";
    let cmp = vec![
        Block::from_string("101100111"),
        Block::from_string("101010111"),
        Block::from_string("010110100"),
        Block::from_string("110101011"),
        Block::from_string("100101111"),
    ];

    assert!(cmp.eq(&parse(data)));
}
