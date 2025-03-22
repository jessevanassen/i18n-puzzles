use std::{io::stdin, ops::Range};

use chrono::{
	DateTime, Datelike, NaiveDate, NaiveTime, TimeDelta, TimeZone, Utc,
	Weekday,
};
use chrono_tz::Tz;
use itertools::{Itertools, MinMaxResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

const YEAR: i32 = 2022;

const WORK_WINDOW: [NaiveTime; 2] = [
	NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
	NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
];

const WHOLE_DAY: [NaiveTime; 2] = [NaiveTime::MIN, NaiveTime::from_hms_opt(23, 59, 59).unwrap()];

fn main() {
	let mut lines = stdin().lines().map(Result::unwrap);

	let offices = (&mut lines)
		.take_while(|line| !line.is_empty())
		.map(|line| parse_entry(&line))
		.map(|(timezone, holidays)| Location {
			timezone,
			holidays,
			time_range: WORK_WINDOW[0]..WORK_WINDOW[1],
		})
		.collect::<Vec<_>>();

	let customers = lines
		.map(|line| parse_entry(&line))
		.map(|(timezone, holidays)| Location {
			timezone,
			holidays,
			time_range: WHOLE_DAY[0]..WHOLE_DAY[1],
		})
		.collect::<Vec<_>>();

	let diff = match customers
		.into_par_iter()
		.map(|ref customer| {
			let offices = &offices;
			all_minutes_in_year()
				.filter(|minute| {
					customer.supports(*minute)
						&& !offices.iter().any(|office| office.supports(*minute))
				})
				.count()
		})
		.collect::<Vec<_>>()
		.into_iter()
		.minmax()
	{
		MinMaxResult::MinMax(min, max) => max - min,
		_ => 0,
	};

	println!("Answer: {diff}");
}

struct Location {
	timezone: Tz,
	holidays: Vec<NaiveDate>,
	time_range: Range<NaiveTime>,
}

impl Location {
	fn supports(&self, minute: DateTime<Utc>) -> bool {
		let local = minute.with_timezone(&self.timezone);
		let date = local.date_naive();
		let time = local.time();

		is_weekday(date) && self.time_range.contains(&time) && !self.holidays.contains(&date)
	}
}

fn all_minutes_in_year() -> impl Iterator<Item = DateTime<Utc>> {
	(0..)
		.map(|i| {
			let base = Utc.with_ymd_and_hms(YEAR, 1, 1, 0, 0, 0).unwrap();
			base + TimeDelta::minutes(i)
		})
		.take_while(|date| date.year() == YEAR)
}

type InputEntry = (Tz, Vec<NaiveDate>);

fn parse_entry(line: &str) -> InputEntry {
	/* Don't care about the name */
	let (_, line) = line.split_once("\t").unwrap();
	let (tz, holidays) = line.split_once("\t").unwrap();
	let tz: Tz = tz.parse().unwrap();
	let holidays = holidays.split(";").map(parse_date).collect();

	(tz, holidays)
}

fn parse_date(date: &str) -> NaiveDate {
	let (day, rest) = date.split_once(" ").unwrap();
	let (month, year) = rest.split_once(" ").unwrap();

	let month = match month {
		"January" => 1,
		"February" => 2,
		"March" => 3,
		"April" => 4,
		"May" => 5,
		"June" => 6,
		"July" => 7,
		"August" => 8,
		"September" => 9,
		"October" => 10,
		"November" => 11,
		"December" => 12,
		other => panic!("Unknown month {other}"),
	};

	NaiveDate::from_ymd_opt(year.parse().unwrap(), month, day.parse().unwrap()).unwrap()
}

fn is_weekday(date: NaiveDate) -> bool {
	!matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}
