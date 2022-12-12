use std::{env, process};

use error_detection::{checksum, crc, simple_parity_check, two_dim_parity_check, Block};

fn main() {
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();

    if args.is_empty() || args.len() < 2 {
        print_help();
        process::exit(0);
    }

    let mode = &args[0];
    let inputs = &args[1..];

    if mode == "spc" {
        spc(inputs);
    } else if mode == "tpc" {
        tpc(&inputs[0]);
    } else if mode == "checksum" {
        checksum(&inputs[0]);
    } else if mode == "crc" {
        crc(&inputs[0]);
    }
}

fn check_length(length: usize, stream: &[Block]) -> bool {
    for block in stream.iter() {
        if block.peek().len() != length {
            return false;
        }
    }

    true
}

fn spc(inputs: &[String]) {
    let a = Block::from_string(&inputs[0]);
    let b = Block::from_string(&inputs[1]);

    let sender = simple_parity_check::encode(&a);

    println!("@Sender");
    println!("{}", sender);

    println!("@Receiver");
    match simple_parity_check::decode(&b) {
        Some(dataword) => println!("{dataword}"),
        None => println!("Dataword: Discarded"),
    }
}

fn tpc(stream: &str) {
    let length: usize = 9;
    let stream = error_detection::parse(stream);

    // Check if the datablocks have the required length
    if !check_length(length, &stream) {
        println!("Datawords must have the same length");
        return;
    }

    let error_count = two_dim_parity_check::count_error(&stream);
    println!("Error count: {}", error_count);
}

fn checksum(stream: &str) {
    let length: usize = 8;
    let stream = error_detection::parse(stream);

    // Check if the datablocks have the required length
    if !check_length(length, &stream) {
        println!("Datawords must have the same length");
        return;
    }

    let decision = checksum::checksum_check(&stream);
    println!("{decision}");
}

fn crc(input: &str) {
    let input = Block::from_string(input);
    let decision = crc::crc_check(&input);

    println!("{decision}");
}

fn print_help() {
    println!(
        "
-----------------------------------
LabEx7: Error Detection Techniques
-----------------------------------
Available commands:
- Simple Parity Check:
    erdet spc \"Input A\" \"Input B\"

- Two-dimensional Parity Check:
    erdet tpc \"data\"

- Checksum:
    erdet checksum \"data\"

- Cyclic Redundancy Check:
    erdet crc \"data\"
"
    );
}
