use error_detection::simple_parity_check;
use error_detection::Block;

#[test]
fn encoded_even() {
    let dataword = Block::from_string("1010");
    let codeword = simple_parity_check::encode(&dataword);
    let cmp = vec![1, 0, 1, 0, 0];

    assert_eq!(cmp, *codeword.peek());
}

#[test]
fn encoded_odd() {
    let dataword = Block::from_string("1110");
    let codeword = simple_parity_check::encode(&dataword);
    let cmp = vec![1, 1, 1, 0, 1];

    assert!(cmp.eq(codeword.peek()));
}

#[test]
fn decoded_even() {
    let codeword = Block::from_string("10100");
    let dataword = simple_parity_check::decode(&codeword);
    let cmp = Some(Block::from_string("1010"));

    assert_eq!(dataword, cmp);
}

#[test]
fn decoded_odd() {
    let codeword = Block::from_string("11101");
    let dataword = simple_parity_check::decode(&codeword);
    let cmp = Some(Block::from_string("1110"));

    assert_eq!(dataword, cmp);
}

#[test]
fn decoded_discarded() {
    let codeword = Block::from_string("11100");
    let dataword = simple_parity_check::decode(&codeword);

    assert_eq!(None, dataword);
}
