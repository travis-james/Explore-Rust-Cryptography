use std::io::{self, Read};

use shift::encipher;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Cipher key 0 to +255
    #[arg(short, long)] // This means short --k or long --key
    key: u8
}

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).expect("Failed to read stdin");

    let args = Args::parse();

    let ciphered_bytes = encipher(&input, &args.key);
    println!("{}", String::from_utf8_lossy(&ciphered_bytes));
}
