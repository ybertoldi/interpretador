pub mod scanner;

use color_eyre::eyre::{Ok, Result};
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use scanner::{Scanner, Token};

fn main() -> Result<()> {
    let v: Vec<_> = env::args().collect();

    if v.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    }

    if v.len() == 2 {
        run_script(&v[1]).unwrap();
    } else {
        run_prompt().unwrap();
    }

    Ok(())
}

fn run_script(v: &str) -> Result<()> {
    let mut buf = fs::read_to_string(v)?;
    run(&mut buf)
}

fn run_prompt() -> Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        print!("rlox > ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }

        run(&line)?;
    }

    Ok(())
}

fn run(buf: &str) -> Result<()> {
    let mut scanner = Scanner::new(buf);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
