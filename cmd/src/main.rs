use std::{
    io::{self, Read, Write},
    process,
};

use clap::{Parser, ValueEnum};
use shift::{crack, decipher, encipher};

#[derive(Parser)]
struct Args {
    /// Input mode: crack, encipher, decipher
     #[arg(short, long, value_enum)]
    mode: Mode,

    /// Cipher key, 0 to +255
    #[arg(short, long)] // This means short --k or long --key
    key: Option<u8>,

    /// Crib for cracking ciphered text, need to use crack mode.
    #[arg(short, long)]
    crib: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Crack,
    Encipher,
    Decipher,
}

fn main() {
    let mut input = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read stdin");

    let args = Args::parse();

    let output: Vec<u8>;
    match args.mode {
        Mode::Encipher => {
            let key = args.key.expect("Key is required for encipher.");
            output = encipher(&input, key);
        }
        Mode::Decipher => {
            let key = args.key.expect("Key is required for encipher.");
            output = decipher(&input, key);
        }
        Mode::Crack => {
            let crib = args.crib.expect("Crib is required for crack.");
            let key = crack(&input, &crib.as_bytes().to_vec());
            if key.is_none() {
                println!("Failed to find key....");
                process::exit(1);
            }
            output = decipher(&input, key.unwrap());
        }
    }

    io::stdout()
        .write_all(&output)
        .expect("Failed to write output");
}
