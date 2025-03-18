use std::{fmt::Display, io::stdin, str::FromStr};

use itertools::Itertools;
use unidecode::{unidecode, unidecode_char};

fn main() {
	let entries: Vec<Entry> = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| Entry::from_str(&line).unwrap())
		.collect();

	let answer = [
		middle(&entries, english_sorting_key),
		middle(&entries, swedish_sorting_key),
		middle(&entries, dutch_sorting_key),
	]
	.into_iter()
	.map(|entry| entry.phone_number.parse::<u64>().unwrap())
	.product::<u64>();

	println!("Answer: {answer}");
}

fn english_sorting_key(s: &str) -> String {
	unidecode(s)
		.chars()
		.flat_map(char::to_lowercase)
		.filter(|ch| ch.is_alphabetic())
		.collect()
}

fn swedish_sorting_key(s: &str) -> String {
	let mut key = String::with_capacity(s.len());

	for ch in s.chars().flat_map(char::to_lowercase) {
		match ch {
			'å' => key.push('{'),
			'ä' | 'æ' => key.push('|'),
			'ö' | 'ø' => key.push('}'),
			ch if ch.is_alphabetic() => {
				key.push_str(unidecode_char(ch));
			}
			_ => { /* skip */ }
		}
	}

	key
}

fn dutch_sorting_key(s: &str) -> String {
	let first_uppercase = s
		.chars()
		.position(char::is_uppercase)
		.expect("Expect last name to have a capital letter somewhere");
	english_sorting_key(&s[first_uppercase..])
}

fn middle(entries: &[Entry], key: fn(&str) -> String) -> &Entry {
	assert!(entries.len() % 2 == 1);

	let sorted = entries
		.iter()
		.sorted_by_key(|entry| key(&entry.last_name))
		.collect::<Vec<_>>();
	sorted[sorted.len() / 2]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
	first_name: String,
	last_name: String,
	phone_number: String,
}

impl FromStr for Entry {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (name, phone_number) = s.split_once(": ").ok_or("Input error: missing ': '")?;
		let (last_name, first_name) = name.split_once(", ").ok_or("Input error: missing ', '")?;

		Ok(Self {
			first_name: first_name.to_string(),
			last_name: last_name.to_string(),
			phone_number: phone_number.to_string(),
		})
	}
}

impl Display for Entry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}, {}: {}",
			self.last_name, self.first_name, self.phone_number
		)
	}
}
