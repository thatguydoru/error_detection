use error_detection::{checksum::checksum_decision, Block};

#[test]
fn checksum_test() {
    let data = vec![
        Block::from_string("10011001"),
        Block::from_string("11100010"),
        Block::from_string("00100100"),
        Block::from_string("10000100"),
        Block::from_string("11011010"),
    ];

    assert_eq!("Accept data", checksum_decision(&data));
}
