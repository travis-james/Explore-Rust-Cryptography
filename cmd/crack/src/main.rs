use std::{
    io::{self, Read, Write},
    process,
};
use clap::Parser;
use shift::{crack, decipher};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    crib: Vec<u8>,
}

fn main() {
    let mut input = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read stdin");

    let args = Args::parse();

    let key = crack(&input, &args.crib);
    if key.is_none() {
        println!("Failed to find key....");
        process::exit(1);
    }
    // Now get the deciphered text..
    let plain_text = decipher(&input, key.unwrap());
    io::stdout()
        .write_all(&plain_text)
        .expect("Failed to write output");
}
