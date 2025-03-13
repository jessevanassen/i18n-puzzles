use std::io::stdin;

use chrono::{DateTime, FixedOffset, Offset as _, TimeDelta, Timelike as _};

fn main() {
	let answer = stdin()
		.lines()
		.map(Result::unwrap)
		.map(|line| {
			let mut columns = line.split_whitespace();

			let datetime: DateTime<FixedOffset> = columns.next().unwrap().parse().unwrap();
			let datetime = deduce_timezone(&datetime);

			let correct_duration: i64 = columns.next().unwrap().parse().unwrap();
			let correct_duration = TimeDelta::minutes(correct_duration);

			let wrong_duration: i64 = columns.next().unwrap().parse().unwrap();
			let wrong_duration = TimeDelta::minutes(wrong_duration);

			datetime - wrong_duration + correct_duration
		})
		.enumerate()
		.map(|(i, datetime)| (i + 1) as u64 * datetime.hour() as u64)
		.sum::<u64>();

	println!("Answer: {answer}");
}

fn deduce_timezone(datetime: &DateTime<FixedOffset>) -> DateTime<chrono_tz::Tz> {
	if is_possible_timezone(datetime, chrono_tz::America::Halifax) {
		datetime.with_timezone(&chrono_tz::America::Halifax)
	} else {
		datetime.with_timezone(&chrono_tz::America::Santiago)
	}
}

fn is_possible_timezone(datetime: &DateTime<FixedOffset>, tz: chrono_tz::Tz) -> bool {
	&datetime.with_timezone(&tz).offset().fix() == datetime.offset()
}