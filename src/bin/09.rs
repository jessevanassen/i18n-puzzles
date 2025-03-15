use std::{collections::BTreeMap, io::stdin};

use chrono::NaiveDate;
use itertools::Itertools;

type DateComponents = [u8; 3];

const _9_11: NaiveDate = NaiveDate::from_ymd_opt(2001, 9, 11).unwrap();

fn main() {
	let input = stdin().lines().map(Result::unwrap);
	let input = parse_input(input);

	let names = input.iter().filter_map(|(name, dates)| {
		let format = DateFormat::FORMATS
			.into_iter()
			.filter(|format| dates.iter().all(|date| format.is_possible(*date)))
			.exactly_one()
			.unwrap();

		let wrote_on_9_11 = dates
			.iter()
			.any(|date| format.parse(*date).unwrap() == _9_11);

		wrote_on_9_11.then_some(name)
	});

	for name in names {
		print!("{name} ");
	}
	println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DateFormat {
	Dmy,
	Mdy,
	Ymd,
	Ydm,
}

impl DateFormat {
	const FORMATS: [Self; 4] = [Self::Dmy, Self::Mdy, Self::Ymd, Self::Ydm];

	fn parse(&self, components: DateComponents) -> Option<NaiveDate> {
		let [y, m, d] = match self {
			DateFormat::Dmy => [components[2], components[1], components[0]],
			DateFormat::Mdy => [components[2], components[0], components[1]],
			DateFormat::Ymd => components,
			DateFormat::Ydm => [components[0], components[2], components[1]],
		};
		let (y, m, d) = (y as i32, m as u32, d as u32);

		let y = if (0..=20).contains(&y) {
			2000 + y
		} else {
			1900 + y
		};
		NaiveDate::from_ymd_opt(y, m, d)
	}

	fn is_possible(&self, components: DateComponents) -> bool {
		self.parse(components).is_some()
	}
}

fn parse_input<I, S>(input: I) -> BTreeMap<String, Vec<DateComponents>>
where
	I: IntoIterator<Item = S>,
	S: AsRef<str>,
{
	use nom::{
		IResult, Parser,
		bytes::complete::tag,
		character::complete::{alpha1, digit1},
		multi::separated_list1,
		sequence::separated_pair,
	};

	fn parse_entry(entry: &str) -> IResult<&str, (DateComponents, Vec<&str>)> {
		fn parse_date_components(input: &str) -> IResult<&str, DateComponents> {
			fn number(input: &str) -> IResult<&str, u8> {
				digit1.map_res(|nr: &str| nr.parse()).parse(input)
			}

			(number, tag("-"), number, tag("-"), number)
				.map(|(x, _, y, _, z)| [x, y, z])
				.parse(input)
		}

		separated_pair(
			parse_date_components,
			tag(": "),
			separated_list1(tag(", "), alpha1),
		)
		.parse(entry)
	}

	let mut result = BTreeMap::<String, Vec<DateComponents>>::new();

	for entry in input {
		let (_, (date_components, names)) = parse_entry(entry.as_ref()).unwrap();

		for name in names {
			if let Some(vec) = result.get_mut(name) {
				vec.push(date_components);
			} else {
				result
					.entry(name.to_string())
					.or_insert(vec![date_components]);
			};
		}
	}

	result
}
