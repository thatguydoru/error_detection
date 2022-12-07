use error_detection::{parse, Block};

#[test]
fn dword_to_vec_bytes() {
    let dword_str = "1010";
    let cmp = vec![1, 0, 1, 0];

    assert!(cmp.eq(Block::new(dword_str).peek()));
}

#[test]
fn cword_to_vec_bytes() {
    let cword_str = "1010";
    let cmp = vec![1, 0, 1, 0];

    assert!(cmp.eq(Block::new(cword_str).peek()));
}

#[test]
fn parse_success() {
    let data = "101100111 101010111 010110100 110101011 100101111";
    let cmp = vec![
        Block::new("101100111"),
        Block::new("101010111"),
        Block::new("010110100"),
        Block::new("110101011"),
        Block::new("100101111"),
    ];

    assert!(cmp.eq(&parse(data)));
}
