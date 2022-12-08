use error_detection::{checksum::checksum_decision, Block};

#[test]
fn checksum_fail() {
    let data = vec![
        Block::from_string("10010001"),
        Block::from_string("11100010"),
        Block::from_string("00100000"),
        Block::from_string("10000101"),
        Block::from_string("11011010"),
    ];

    assert_eq!("Checksum error detected", checksum_decision(&data));
}

#[test]
fn checksum_pre_test() {
    let data = vec![
        Block::from_string("10011001"),
        Block::from_string("11100010"),
        Block::from_string("00100100"),
        Block::from_string("10000100"),
        Block::from_string("11011010"),
    ];

    assert_eq!("Accept data", checksum_decision(&data));
}

#[test]
fn checksum_post_test() {
    // 10011001 11100010 00100100 10000100 11011010
    let data = vec![
        Block::from_string("10011001"),
        Block::from_string("11100010"),
        Block::from_string("00100100"),
        Block::from_string("10000100"),
        Block::from_string("11011010"),
    ];

    assert_eq!("Accept data", checksum_decision(&data));
}
