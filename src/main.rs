use std::io::{stdin, BufRead};

mod interpret;
mod lex;
mod parse;

fn main() {
	for line in stdin().lock().lines() {
		let line = line.unwrap();
		let tokens = match lex::lex(&line) {
			Ok(tokens) => tokens,
			Err(err) => {
				eprintln!("lex error: {err}");
				continue;
			}
		};
		let expr = match parse::parse(&tokens) {
			Ok(expr) => expr,
			Err(err) => {
				eprintln!("syntax error: {err}");
				continue;
			}
		};
		match interpret::interpret(expr) {
			Ok(n) => println!("{n}"),
			Err(err) => eprintln!("runtime error: {err}"),
		}
	}
}
