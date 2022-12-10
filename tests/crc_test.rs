use error_detection::{crc::crc_check, Block};

#[test]
fn crc_accept_test() {
    let data = Block::from_string("1001110");

    assert_eq!(&"Accept data", &crc_check(&data));
}

#[test]
fn crc_fail_test() {
    let data = Block::from_string("1011110");

    assert_ne!(&"Accept data", &crc_check(&data));
}
