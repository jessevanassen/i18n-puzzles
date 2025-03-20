use std::io::stdin;

fn main() {
	let answer = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let (left, right) = line.split_once(" × ").expect("Expect two values");

			[left, right]
				.into_iter()
				.map(|value| {
					let chars = value.chars().collect::<Vec<_>>();
					let number = parse_number(&chars[..chars.len() - 1]).unwrap();
					let unit = parse_unit(*chars.last().unwrap()).unwrap();

					number * unit
				})
				.product::<u128>()
		})
		.map(|area_in_mo| area_in_mo / 1089000000)
		.sum::<u128>();

	println!("Answer: {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NumberPart {
	Base(u128),
	Power(u128),
	Myriad(u128),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
#[error("Invalid numeric character {0}")]
struct InvalidCharacterError(char);

impl TryFrom<char> for NumberPart {
	type Error = InvalidCharacterError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'一' => Self::Base(1),
			'二' => Self::Base(2),
			'三' => Self::Base(3),
			'四' => Self::Base(4),
			'五' => Self::Base(5),
			'六' => Self::Base(6),
			'七' => Self::Base(7),
			'八' => Self::Base(8),
			'九' => Self::Base(9),

			'十' => Self::Power(10),
			'百' => Self::Power(100),
			'千' => Self::Power(1000),

			'万' => Self::Myriad(10000),
			'億' => Self::Myriad(100000000),

			other => Err(InvalidCharacterError(other))?,
		})
	}
}

fn parse_number(value: &[char]) -> Result<u128, InvalidCharacterError> {
	let mut acc = [0, 0, 0];

	for item in value.iter().copied().map(NumberPart::try_from) {
		match item? {
			NumberPart::Base(n) => {
				acc[1] += acc[2];
				acc[2] = n;
			}
			NumberPart::Power(n) => {
				acc[1] += acc[2].max(1) * n;
				acc[2] = 0;
			}
			NumberPart::Myriad(n) => {
				acc[0] += (acc[1] + acc[2]).max(1) * n;
				acc[1] = 0;
				acc[2] = 0;
			}
		}
	}

	Ok(acc.into_iter().sum())
}

fn parse_unit(value: char) -> Result<u128, InvalidCharacterError> {
	Ok(match value {
		'毛' => 1,
		'厘' => 10,
		'分' => 100,
		'寸' => 1000,
		'尺' => 10_000,
		'間' => 6 * 10_000,
		'丈' => 10 * 10_000,
		'町' => 360 * 10_000,
		'里' => 12_960 * 10_000,
		other => Err(InvalidCharacterError(other))?,
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_number() {
		for (expected, input) in [
			(300, "三百"),
			(321, "三百二十一"),
			(4_000, "四千"),
			(50_000, "五万"),
			(99_999, "九万九千九百九十九"),
			(420_042, "四十二万四十二"),
			(987_654_321, "九億八千七百六十五万四千三百二十一"),
			(612, "六百十二"),
		] {
			assert_eq!(
				parse_number(&input.chars().collect::<Vec<_>>()),
				Ok(expected)
			)
		}
	}
}
