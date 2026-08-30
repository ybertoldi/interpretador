pub mod ast;
pub mod interpreter;
pub mod literal_type;
pub mod parser;
pub mod scanner;

use color_eyre::eyre::{Ok, Result};
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use crate::{interpreter::Interpreter, parser::Parser};
use scanner::{Scanner, Token};

struct Options {
    show_tokens: bool,
    show_grammar: bool,
}
fn main() -> Result<()> {
    let mut opts = Options {
        show_tokens: false,
        show_grammar: false,
    };
    let v: Vec<_> = env::args().collect();

    if v.len() > 4 {
        println!("Usage: rlox [script] [-t] [-g]");
        exit(64);
    }

    let mut filename = "";
    for value in &v[1..] {
        match value.as_str() {
            "-t" => {
                opts.show_tokens = true;
            }
            "-g" => {
                opts.show_grammar = true;
            }

            other => filename = other,
        };
    }

    let mut interpreter = Interpreter::new();
    if !filename.is_empty() {
        run_script(filename, opts, &mut interpreter).unwrap();
    } else {
        run_prompt(opts, &mut interpreter).unwrap();
    }

    Ok(())
}

fn run_script(v: &str, opts: Options, interpreter: &mut Interpreter) -> Result<()> {
    let mut buf = fs::read_to_string(v)?;
    run(&mut buf, &opts, interpreter)
}

fn run_prompt(opts: Options, interpreter: &mut Interpreter) -> Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        print!("rlox > ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }

        run(&line, &opts, interpreter)?;
    }

    Ok(())
}

fn run(buf: &str, opts: &Options, interpreter: &mut Interpreter) -> Result<()> {
    let mut scanner = Scanner::new(buf);
    let tokens: Vec<Token> = scanner.scan_tokens();

    if opts.show_tokens {
        println!("--TOKENS--");
        for token in &tokens {
            println!("{:?}", token);
        }
        println!("--END OF TOKENS--\n");
    }
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if opts.show_grammar {
        println!("--AST--");
        println!("{:?}", ast);
        println!("--END OF AST--\n");
    }

    let result = interpreter.run(&ast.0);

    if let Some(res) = result {
        println!("{:?}", res);
    }

    Ok(())
}
