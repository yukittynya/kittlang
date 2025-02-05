use std::env;
use std::fs::File;
use std::io;

mod tokens;
mod errors;
mod scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: kittlang FILE\n");
        return Ok(());
    }

    let mut file = File::open(&args[1]);

    Ok(())
}
