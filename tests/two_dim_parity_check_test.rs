use error_detection::{two_dim_parity_check::count_error, Block};

#[test]
fn no_errors() {
    let parsed_data = vec![
        Block::new("101100111"),
        Block::new("101010111"),
        Block::new("010110100"),
        Block::new("110101011"),
        Block::new("100101111"),
    ];

    assert_eq!(0, count_error(parsed_data))
}

#[test]
fn one_error() {
    // error in index(1,4)
    let parsed_data = vec![
        Block::new("101100111"),
        Block::new("101000111"),
        Block::new("010110100"),
        Block::new("110101011"),
        Block::new("100101111"),
    ];

    assert_eq!(1, count_error(parsed_data))
}

#[test]
fn two_errors() {
    // errors in indexes (0,5), (2,3)
    let parsed_data = vec![
        Block::new("101101111"),
        Block::new("101010111"),
        Block::new("010010100"),
        Block::new("110101011"),
        Block::new("100101111"),
    ];

    assert_eq!(2, count_error(parsed_data))
}

#[test]
fn edge_case_three_errors() {
    // errors in indexes (1,5), (1,3), (3,3)
    let parsed_data = vec![
        Block::new("101100111"),
        Block::new("101111111"),
        Block::new("010110100"),
        Block::new("110001011"),
        Block::new("100101111"),
    ];

    assert_eq!(1, count_error(parsed_data))
}

#[test]
fn four_errors() {
    // error in indexes (1,4), (3,4), (1,6), (3,6)
    let parsed_data = vec![
        Block::new("101100111"),
        Block::new("101000011"),
        Block::new("010110100"),
        Block::new("110111111"),
        Block::new("100101111"),
    ];

    assert_eq!(0, count_error(parsed_data))
}

#[test]
fn small_data() {
    let data = vec![
        Block::new("10100"),
        Block::new("11101"),
        Block::new("10010"),
        Block::new("11011"),
    ];

    assert_eq!(0, count_error(data));
}
