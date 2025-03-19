use std::io::stdin;

use i18n_puzzles::crossword::parse_crossword;
use itertools::Itertools;

fn main() {
	let mut input = stdin().lines().map(Result::unwrap);

	let words = (&mut input)
		.take_while(|line| !line.is_empty())
		.enumerate()
		.map(|(index, word)| match index + 1 {
			index if index % 15 == 0 => decode_double_miscode(&word),
			index if index % 3 == 0 || index % 5 == 0 => decode_miscode(&word),
			_ => word,
		})
		.collect::<Vec<_>>();

	let answer = parse_crossword(input)
		.map(|entry| {
			words
				.iter()
				.enumerate()
				.filter_map(|(i, word)| entry.matches(word).then_some(i + 1))
				.exactly_one()
				.unwrap()
		})
		.sum::<usize>();

	println!("Answer: {answer}");
}

fn decode_miscode(str: &str) -> String {
	let bytes = str.chars().map(|char| char.try_into().unwrap()).collect();
	String::from_utf8(bytes).unwrap()
}

fn decode_double_miscode(str: &str) -> String {
	decode_miscode(&decode_miscode(str))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decode_miscode() {
		assert_eq!(decode_miscode("religiÃ«n"), "religiën");
		assert_eq!(decode_miscode("kÃ¼rst"), "kürst");
		assert_eq!(decode_miscode("roekoeÃ«n"), "roekoeën");
	}

	#[test]
	fn test_decode_double_miscode() {
		assert_eq!(decode_double_miscode("pugilarÃÂ£o"), "pugilarão");
	}
}
