use std::{env, process::exit};

use error_detection::{simple_parity_check, Block, two_dim_parity_check, checksum, crc};

fn main() {
    let args = env::args().into_iter().skip(1).collect::<Vec<String>>();

    if args.is_empty() || args.len() >= 2 {
        print_help();
        exit(0);
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
    let stream = error_detection::parse(stream);
    let error_count = two_dim_parity_check::count_error(&stream);

    println!("Error count: {}", error_count);
}

fn checksum(stream: &str) {
    let stream = error_detection::parse(stream);
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
