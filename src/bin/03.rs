use std::io::stdin;

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
	(4..=12).contains(&password.chars().count())
		&& password.chars().any(|ch| ch.is_ascii_digit())
		&& password.chars().any(|ch| ch.is_lowercase())
		&& password.chars().any(|ch| ch.is_uppercase())
		&& !password.is_ascii()
}
