extern crate plsqlfmt;

use std::io::{self, Read};

pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", plsqlfmt::lexer::read_str(&buffer));
}