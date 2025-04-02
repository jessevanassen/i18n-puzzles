use std::error::Error;
use std::fmt::Display;
use std::io::stdin;
use std::iter::{self, Peekable};

const RLI: char = '\u{2067}';
const LRI: char = '\u{2066}';
const PDI: char = '\u{2069}';

fn main() {
	let answer = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let tokens = tokenize(&line).map(|token| token.unwrap());
			parse(tokens).unwrap()
		})
		.map(|expression| {
			let fst = expression.evaluate();
			let snd = expression.factor_direction_changes().evaluate();
			(snd - fst).abs()
		})
		.sum::<f64>();
	dbg!(answer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
	Plus,
	Minus,
	Star,
	Slash,
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let op = match self {
			Operator::Plus => "+",
			Operator::Minus => "-",
			Operator::Star => "*",
			Operator::Slash => "/",
		};
		write!(f, "{}", op)
	}
}

impl Operator {
	pub fn apply(&self, left: f64, right: f64) -> f64 {
		use std::ops::{Add, Div, Mul, Sub};

		let op = match self {
			Operator::Plus => f64::add,
			Operator::Minus => f64::sub,
			Operator::Star => f64::mul,
			Operator::Slash => f64::div,
		};
		op(left, right)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	Lri,
	Rli,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expression {
	Value(i32),
	Operation {
		operator: Operator,
		left: Box<Expression>,
		right: Box<Expression>,
	},
	Direction {
		direction: Direction,
		inner: Box<Expression>,
	},
}

impl Expression {
	pub fn evaluate(&self) -> f64 {
		match self {
			Expression::Value(v) => *v as f64,
			Expression::Operation {
				operator,
				left,
				right,
			} => {
				let left = left.evaluate();
				let right = right.evaluate();
				operator.apply(left, right)
			}
			Expression::Direction { inner, .. } => inner.evaluate(),
		}
	}

	pub fn factor_direction_changes(&self) -> Expression {
		fn factor_direction_changes(expr: &Expression, right_to_left: bool) -> Expression {
			match expr {
				v @ Expression::Value(_) => v.clone(),
				Expression::Operation {
					operator,
					left,
					right,
				} => {
					let left = factor_direction_changes(left, right_to_left);
					let right = factor_direction_changes(right, right_to_left);

					if right_to_left {
						Expression::Operation {
							operator: *operator,
							left: Box::new(right),
							right: Box::new(left),
						}
					} else {
						Expression::Operation {
							operator: *operator,
							left: Box::new(left),
							right: Box::new(right),
						}
					}
				}
				Expression::Direction { direction, inner } => {
					let direction = match direction {
						Direction::Lri => false,
						Direction::Rli => true,
					};
					factor_direction_changes(inner, direction)
				}
			}
		}

		factor_direction_changes(self, false)
	}
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Value(v) => write!(f, "{v}"),
			Expression::Operation {
				operator,
				left,
				right,
			} => {
				write!(f, "({} {} {})", left, operator, right)
			}
			Expression::Direction { direction, inner } => {
				let direction = match direction {
					Direction::Lri => '⏵',
					Direction::Rli => '⏴',
				};
				write!(f, "{direction}{inner}⏶")
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
	Number(i32),
	Operator(Operator),
	Paren { open: bool },
	Direction(Option<Direction>),
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("Tokenization error at index {index}")]
struct TokenizeError {
	index: usize,
}

fn tokenize(input: &str) -> impl Iterator<Item = Result<Token, TokenizeError>> {
	let mut chars = input.char_indices().peekable();

	iter::from_fn(move || {
		while chars.peek().is_some_and(|(_, ch)| ch.is_whitespace()) {
			chars.next();
		}

		Some(match chars.next()? {
			(_, '+') => Ok(Token::Operator(Operator::Plus)),
			(_, '-') => Ok(Token::Operator(Operator::Minus)),
			(_, '*') => Ok(Token::Operator(Operator::Star)),
			(_, '/') => Ok(Token::Operator(Operator::Slash)),
			(_, '(') => Ok(Token::Paren { open: true }),
			(_, ')') => Ok(Token::Paren { open: false }),
			(_, LRI) => Ok(Token::Direction(Some(Direction::Lri))),
			(_, RLI) => Ok(Token::Direction(Some(Direction::Rli))),
			(_, PDI) => Ok(Token::Direction(None)),
			(_, d) if d.is_numeric() => {
				fn to_number(ch: char) -> u32 {
					ch as u32 - b'0' as u32
				}

				let mut v = to_number(d);

				while let Some(&(_, ch)) = chars.peek() {
					if !ch.is_numeric() {
						break;
					}

					v = v * 10 + to_number(ch);
					chars.next();
				}

				Ok(Token::Number(v as i32))
			}
			(index, _) => Err(TokenizeError { index }),
		})
	})
}

fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Expression, Box<dyn Error>> {
	fn parse_expression(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		parse_add(tokens)
	}

	fn parse_add(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		let mut left = parse_mul(tokens)?;

		while let Some(&Token::Operator(operator)) = tokens.peek() {
			if matches!(operator, Operator::Plus | Operator::Minus) {
				break;
			}

			tokens.next().unwrap();
			let right = parse_mul(tokens)?;

			left = Expression::Operation {
				operator,
				left: Box::new(left),
				right: Box::new(right),
			};
		}

		Ok(left)
	}

	fn parse_mul(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		let mut left = parse_direction(tokens)?;

		while let Some(&Token::Operator(operator)) = tokens.peek() {
			if matches!(operator, Operator::Star | Operator::Slash) {
				break;
			}

			tokens.next().unwrap();
			let right = parse_direction(tokens)?;

			left = Expression::Operation {
				operator,
				left: Box::new(left),
				right: Box::new(right),
			};
		}

		Ok(left)
	}

	fn parse_direction(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		if let Some(&Token::Direction(direction)) = tokens.peek() {
			let Some(direction) = direction else {
				return Err("Expected Direction::Lri or Direction::Rli".into());
			};
			tokens.next().unwrap();

			let expr = parse_expression(tokens)?;

			if !matches!(tokens.next(), None | Some(Token::Direction(None))) {
				return Err("Expected Direction::Pdi".into());
			}

			Ok(Expression::Direction {
				direction,
				inner: Box::new(expr),
			})
		} else {
			parse_group(tokens)
		}
	}

	fn parse_group(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		if tokens.peek() == Some(&Token::Paren { open: true }) {
			tokens.next().unwrap();

			let expr = parse_expression(tokens)?;

			let next = tokens.next();
			if !matches!(next, Some(Token::Paren { open: false })) {
				return Err("Expected closing paren".into());
			}

			Ok(expr)
		} else {
			parse_value(tokens)
		}
	}

	fn parse_value(
		tokens: &mut Peekable<impl Iterator<Item = Token>>,
	) -> Result<Expression, Box<dyn Error>> {
		match tokens.next() {
			Some(Token::Number(n)) => Ok(Expression::Value(n)),
			_ => Err("Expected value".into()),
		}
	}

	let mut tokens = tokens.into_iter().peekable();
	parse_expression(&mut tokens)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize() {
		assert_eq!(
			tokenize(&format!("1+2*(3/4) - {LRI} 567 {RLI} {PDI}"))
				.collect::<Result<Vec<_>, _>>()
				.unwrap(),
			vec![
				Token::Number(1),
				Token::Operator(Operator::Plus),
				Token::Number(2),
				Token::Operator(Operator::Star),
				Token::Paren { open: true },
				Token::Number(3),
				Token::Operator(Operator::Slash),
				Token::Number(4),
				Token::Paren { open: false },
				Token::Operator(Operator::Minus),
				Token::Direction(Some(Direction::Lri)),
				Token::Number(567),
				Token::Direction(Some(Direction::Rli)),
				Token::Direction(None),
			]
		);
	}

	mod parse {
		use super::*;

		fn parse(input: &str) -> Expression {
			super::parse(tokenize(input).collect::<Result<Vec<_>, _>>().unwrap()).unwrap()
		}

		#[test]
		fn test_parse_base_number() {
			assert_eq!(Expression::Value(123), parse("123"));
			assert_eq!(Expression::Value(123), parse("\t123"));
			assert_eq!(Expression::Value(123), parse("123 "));
			assert_eq!(Expression::Value(123), parse("\t 123     "));
			assert_eq!(Expression::Value(123), parse("  \t   123   \t  "));
		}

		#[test]
		fn test_parse_group() {
			assert_eq!(Expression::Value(1), parse("(1)"));
			assert_eq!(Expression::Value(1), parse(" ( 1 ) "));
			assert_eq!(Expression::Value(1), parse(" ( ( ( 1 ) ) ) "));
		}

		#[test]
		fn test_parse_operation() {
			assert_eq!(
				Expression::Operation {
					operator: Operator::Plus,
					left: Box::new(Expression::Value(1)),
					right: Box::new(Expression::Value(2)),
				},
				parse("1 + 2")
			);
		}
	}
}
