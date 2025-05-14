use std::env;
use std::fs;
use std::io;

use crate::scanner::Scanner;

mod expression;
mod parser;
mod scanner;
mod tokens;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: kittlang FILE\n");
        return Ok(());
    }

    let source: String = fs::read_to_string(&args[1])?;
    let mut scanner = Scanner::new();

    scanner.scan_tokens(source);
    scanner.print_tokens();

    Ok(())
}
