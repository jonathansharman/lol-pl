use std::io::{stdin, BufRead};

mod lex;

fn main() {
	for line in stdin().lock().lines() {
		let line = line.unwrap();
		match lex::lex(&line) {
			Ok(tokens) => println!("lexed: {tokens:?}"),
			Err(err) => eprintln!("lex error: {err}"),
		}
	}
}
