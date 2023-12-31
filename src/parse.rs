use crate::lex::Token;

#[derive(Debug)]
pub enum Expr {
	Number(i32),
	Variable(char),
	Sum(Box<(Expr, Expr)>),
	Diff(Box<(Expr, Expr)>),
	Product(Box<(Expr, Expr)>),
	Quotient(Box<(Expr, Expr)>),
	Negation(Box<Expr>),
	Assignment(char, Box<Expr>),
}

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
	let (expr, rest) = parse_expr(tokens)?;
	if !rest.is_empty() {
		return Err(format!("unexpected input after expression: {rest:?}"));
	}
	Ok(expr)
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
	if let Some((&Token::Variable(lhs), after_var)) = tokens.split_first() {
		if let Some((Token::Assign, after_assign)) = after_var.split_first() {
			let (rhs, rest) = parse_math_expr(after_assign)?;
			let assignment = Expr::Assignment(lhs, Box::new(rhs));
			return Ok((assignment, rest));
		}
	}
	parse_math_expr(tokens)
}

fn parse_math_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
	let (mut term, mut rest) = parse_term(tokens)?;
	while let Some((op, after_op)) = rest.split_first() {
		match op {
			Token::Plus => {
				let (operand, after_operand) = parse_term(after_op)?;
				term = Expr::Sum(Box::new((term, operand)));
				rest = after_operand;
			}
			Token::Minus => {
				let (operand, after_operand) = parse_term(after_op)?;
				term = Expr::Diff(Box::new((term, operand)));
				rest = after_operand;
			}
			_ => break,
		}
	}
	Ok((term, rest))
}

fn parse_term(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
	let (mut factor, mut rest) = parse_factor(tokens)?;
	while let Some((op, after_op)) = rest.split_first() {
		match op {
			Token::Divide => {
				let (operand, after_operand) = parse_term(after_op)?;
				factor = Expr::Quotient(Box::new((factor, operand)));
				rest = after_operand;
			}
			Token::Times => {
				let (operand, after_operand) = parse_term(after_op)?;
				factor = Expr::Product(Box::new((factor, operand)));
				rest = after_operand;
			}
			_ => break,
		}
	}
	Ok((factor, rest))
}

fn parse_factor(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
	match tokens.first() {
		Some(Token::Variable(x)) => Ok((Expr::Variable(*x), &tokens[1..])),
		Some(Token::Number(n)) => Ok((Expr::Number(*n), &tokens[1..])),
		Some(Token::Minus) => {
			let (expr, rest) = parse_expr(&tokens[1..])?;
			Ok((Expr::Negation(Box::new(expr)), rest))
		}
		Some(Token::LeftParen) => {
			let (expr, rest) = parse_expr(&tokens[1..])?;
			if let Some(Token::RightParen) = rest.first() {
				Ok((expr, &rest[1..]))
			} else {
				Err("missing closing parenthesis".to_string())
			}
		}
		_ => Err(format!("expected factor, got {tokens:?}")),
	}
}
