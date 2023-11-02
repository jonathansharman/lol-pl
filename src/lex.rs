#[derive(Debug)]
pub enum Token {
	LeftParen,
	RightParen,
	Plus,
	Minus,
	Times,
	Divide,
	Number(i32),
}

enum State {
	Start,
	Number { start: usize },
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
	let mut state = State::Start;
	let mut tokens = Vec::new();

	for (i, c) in input.chars().enumerate() {
		match state {
			State::Start => match c {
				'(' => tokens.push(Token::LeftParen),
				')' => tokens.push(Token::RightParen),
				'+' => tokens.push(Token::Plus),
				'-' => tokens.push(Token::Minus),
				'*' => tokens.push(Token::Times),
				'/' => tokens.push(Token::Divide),
				'0'..='9' => state = State::Number { start: i },
				_ => {
					return Err(format!(
						"unexpected character {c} at index {i}"
					))
				}
			},
			State::Number { start } => match c {
				'0'..='9' => (),
				'(' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::LeftParen);
					state = State::Start;
				}
				')' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::RightParen);
					state = State::Start;
				}
				'+' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::Plus);
					state = State::Start;
				}
				'-' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::Minus);
					state = State::Start;
				}
				'*' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::Times);
					state = State::Start;
				}
				'/' => {
					tokens
						.push(Token::Number(input[start..i].parse().unwrap()));
					tokens.push(Token::Divide);
					state = State::Start;
				}
				_ => {
					return Err(format!(
						"unexpected character {c} at index {i}"
					))
				}
			},
		}
	}
	if let State::Number { start } = state {
		tokens.push(Token::Number(input[start..].parse().unwrap()));
	}
	Ok(tokens)
}
