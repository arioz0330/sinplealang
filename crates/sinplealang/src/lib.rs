#![no_std]
#![feature(in_band_lifetimes)]

extern crate alloc;

mod binding_def;
mod env;
mod expr;
mod stmt;
mod utils;
mod val;

pub use env::Env;
pub use val::Val;

use alloc::string::{String, ToString};

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("Error: Input was not fully consumed by parser".to_string())
    }
}
