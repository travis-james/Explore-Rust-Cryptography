use std::io::{self, Read};

use clap::Parser;
use shift::decipher;

#[derive(Parser)]
struct Args {
    /// Cipher key 0 to +255
    #[arg(short, long)] // This means short --k or long --key
    key: u8,
}

fn main() {
    let mut input = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read stdin");

    let args = Args::parse();

    let ciphered_bytes = decipher(&input, args.key);
    io::stdout()
        .write_all(&ciphered_bytes)
        .expect("Failed to write output");
}
