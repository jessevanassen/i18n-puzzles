use std::io::stdin;

use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Tz;
use itertools::Itertools;

fn main() {
	println!(
		"Total travel time: {} minutes",
		stdin()
			.lines()
			.map(Result::unwrap)
			.chunks(3)
			.into_iter()
			.map(|mut chunk| {
				let departure = chunk.next().unwrap();
				let departure = parse_entry(&departure);

				let arrival = chunk.next().unwrap();
				let arrival = parse_entry(&arrival);

				let duration = arrival - departure;

				duration.num_minutes()
			})
			.sum::<i64>()
	);
}

fn parse_entry(entry: &str) -> DateTime<chrono_tz::Tz> {
	use nom::{
		IResult, Parser as _,
		bytes::complete::take_while1,
		character::complete::space1,
		combinator::{map_res, rest},
	};

	fn word(s: &str) -> IResult<&str, &str> {
		take_while1(|c: char| !c.is_ascii_whitespace())(s)
	}

	fn tz(s: &str) -> IResult<&str, Tz> {
		map_res(word, |s| s.parse()).parse(s)
	}

	fn datetime(s: &str) -> IResult<&str, NaiveDateTime> {
		map_res(rest, |s| {
			NaiveDateTime::parse_from_str(s, "%b %d, %Y, %H:%M")
		})
		.parse(s)
	}

	let (_, _, tz, _, datetime) = (word, space1, tz, space1, datetime)
		.parse(entry)
		.expect("Expected valid puzzle input")
		.1;

	datetime.and_local_timezone(tz).single().unwrap()
}
