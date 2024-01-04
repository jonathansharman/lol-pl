use std::collections::HashMap;

use crate::parse::Expr;

pub fn interpret(
	expr: Expr,
	env: &mut HashMap<char, i32>,
) -> Result<i32, String> {
	match expr {
		Expr::Number(n) => Ok(n),
		Expr::Variable(x) => match env.get(&x) {
			Some(value) => Ok(*value),
			None => Err(format!("{x} is undefined")),
		},
		Expr::Sum(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a, env)? + interpret(b, env)?)
		}
		Expr::Diff(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a, env)? - interpret(b, env)?)
		}
		Expr::Product(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a, env)? * interpret(b, env)?)
		}
		Expr::Quotient(operands) => {
			let (a, b) = *operands;
			let divisor = interpret(b, env)?;
			if divisor == 0 {
				Err("division by zero".to_string())
			} else {
				Ok(interpret(a, env)? / divisor)
			}
		}
		Expr::Negation(operand) => Ok(-interpret(*operand, env)?),
		Expr::Assignment(lhs, rhs) => {
			let value = interpret(*rhs, env)?;
			env.insert(lhs, value);
			Ok(value)
		}
	}
}
