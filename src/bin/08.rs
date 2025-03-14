use std::io::stdin;

use itertools::Itertools as _;
use unicode_normalization::UnicodeNormalization as _;

fn main() {
	println!(
		"Answer: {}",
		stdin()
			.lines()
			.map(Result::unwrap)
			.filter(|password| is_valid(password))
			.count()
	);
}

fn is_valid(password: &str) -> bool {
	let password = deaccent(password);

	(4..=12).contains(&password.chars().count())
		&& password.chars().any(|ch| ch.is_ascii_digit())
		&& password.chars().any(is_vowel)
		&& password.chars().any(is_consonant)
		&& !has_recurring_letters(&password)
}

fn deaccent(str: &str) -> String {
	str.chars().map(|ch| ch.nfd().next().unwrap()).collect()
}

fn is_vowel(ch: char) -> bool {
	matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_consonant(ch: char) -> bool {
	ch.is_alphabetic() && !is_vowel(ch)
}

fn has_recurring_letters(password: &str) -> bool {
	password
		.chars()
		.filter(|ch| ch.is_ascii_alphabetic())
		.map(|ch| ch.to_ascii_lowercase())
		.counts()
		.into_iter()
		.any(|(_, counts)| counts > 1)
}
