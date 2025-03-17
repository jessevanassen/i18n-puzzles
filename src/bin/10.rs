use std::{collections::BTreeMap, io::stdin};

use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use unicode_normalization::UnicodeNormalization;

fn main() {
	let mut lines = stdin().lines().map(Result::unwrap);

	let passwords: BTreeMap<String, String> = (&mut lines)
		.take_while(|line| !line.is_empty())
		.map(|line| {
			let (name, pw) = line.split_once(' ').unwrap();
			(name.to_string(), pw.to_string())
		})
		.collect();

	let valid_attempts = lines
		.filter(|line| {
			let (name, pw) = line.split_once(' ').unwrap();
			let Some(stored) = passwords.get(name) else {
				return false;
			};

			possible_representations(pw)
				.into_par_iter()
				.any(|repr| bcrypt::verify(repr, stored).unwrap())
		})
		.count();

	println!("Valid attempts: {valid_attempts}");
}

fn possible_representations(password: &str) -> Vec<String> {
	fn possible_representations(acc: &mut Vec<String>, mut prefix: String, remaining: &[char]) {
		match remaining {
			[] => {
				acc.push(prefix);
			}
			[ch, remaining @ ..] => {
				let mut decomposed = ch.nfd();
				if let (Some(fst), Some(snd)) = (decomposed.next(), decomposed.next()) {
					let mut prefix = prefix.clone();
					prefix.push(fst);
					prefix.push(snd);
					possible_representations(acc, prefix, remaining);
				}

				prefix.push(*ch);
				possible_representations(acc, prefix, remaining);
			}
		}
	}

	let chars = password.nfc().collect::<Vec<_>>();
	let mut acc = Vec::new();

	possible_representations(&mut acc, String::new(), &chars[..]);

	acc
}
