use super::error::*;
use crate::ast::lexer::Lexer;
use color_print::cprint;
use std::{
    fs,
    io::{self, BufRead, Write},
};

pub fn repl() -> Result<()> {
    loop {
        cprint!("<s,#C0C0C0>Na</> <#8989FF>>></> ");
        io::stdout().lock().flush().unwrap();

        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        if let Err(e) = eval("<stdin>", &line) {
            eprintln!("{e:?}")
        }
    }
}

pub fn script(name: &str) -> Result<()> {
    let content = fs::read_to_string(name).map_err(|e| Error::FileErr(e.to_string()))?;
    eval(name, &content)
}

fn eval(file: &str, source: &str) -> Result<()> {
    let tokens = Lexer::new(file, source).tokenize()?;
    println!("{tokens:?}");
    Ok(())
}
