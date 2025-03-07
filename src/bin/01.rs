use std::io::stdin;

fn main() {
	let total_cost = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| cost(&line))
		.sum::<u32>();

	println!("Total cost: {total_cost}");
}

fn cost(input: &str) -> u32 {
	const SMS_THRESHOLD: usize = 160;
	const TWEET_THRESHOLD: usize = 140;

	let bytes = input.len();
	let characters = input.chars().count();

	match (bytes <= SMS_THRESHOLD, characters <= TWEET_THRESHOLD) {
		(true, true) => 13,
		(true, false) => 11,
		(false, true) => 7,
		(false, false) => 0,
	}
}
