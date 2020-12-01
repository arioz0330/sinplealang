// #![no_std]

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = sinplealang::Env::default();

    loop {
        write!(stdout, "Sinplealang REPL → ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;


        match run(input.trim(), &mut env) {
            Ok(Some(val)) => {
                writeln!(stdout, "{}", val)?
            },
            Ok(None) => {},
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }

        input.clear();
    }
}

fn run(input: &str, env: &mut sinplealang::Env) -> Result<Option<sinplealang::Val>, String> {
    let parse = sinplealang::parse(input).map_err(|msg| format!("Error: (parse) {}", msg))?;

    let evaluated = parse
    .eval(env)
    .map_err(|msg| format!("Error: (eval) {}", msg))?;

    if evaluated == sinplealang::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}