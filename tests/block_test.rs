use error_detection::Block;

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
