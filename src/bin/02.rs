use std::{io::stdin, str::FromStr as _};

use chrono::{DateTime, Utc};
use itertools::Itertools;

fn main() {
	let answer = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|str| DateTime::<Utc>::from_str(&str).unwrap())
		.counts()
		.into_iter()
		.filter_map(|(instant, count)| (count >= 4).then_some(instant))
		.exactly_one()
		.expect("Expected only one instant that occurs four or more times in the input");

	println!("{}", answer.fixed_offset().to_rfc3339());
}
