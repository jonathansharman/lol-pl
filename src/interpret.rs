use crate::parse::Expr;

pub fn interpret(expr: Expr) -> Result<i32, String> {
	match expr {
		Expr::Number(n) => Ok(n),
		Expr::Sum(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a)? + interpret(b)?)
		}
		Expr::Diff(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a)? - interpret(b)?)
		}
		Expr::Product(operands) => {
			let (a, b) = *operands;
			Ok(interpret(a)? * interpret(b)?)
		}
		Expr::Quotient(operands) => {
			let (a, b) = *operands;
			let divisor = interpret(b)?;
			if divisor == 0 {
				Err("division by zero".to_string())
			} else {
				Ok(interpret(a)? / divisor)
			}
		}
		Expr::Negation(operand) => Ok(-interpret(*operand)?),
	}
}
