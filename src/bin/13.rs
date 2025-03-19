use core::str;
use std::{
	char::{DecodeUtf16Error, decode_utf16},
	io::stdin,
};

use i18n_puzzles::crossword::parse_crossword;
use itertools::Itertools as _;

fn main() {
	let mut lines = stdin().lines().map(Result::unwrap);

	let inputs = (&mut lines)
		.take_while(|line| !line.is_empty())
		.enumerate()
		.flat_map(|(i, line)| parse_line(&line).map(|str| (i, str)).collect::<Vec<_>>())
		.filter(|(_, word)| word.chars().all(|ch| ch.is_alphabetic()))
		.collect::<Vec<_>>();

	let answer = parse_crossword(lines)
		.map(|entry| {
			inputs
				.iter()
				.filter_map(|(i, word)| entry.matches(word).then_some(i + 1))
				.exactly_one()
				.unwrap()
		})
		.sum::<usize>();

	println!("Answer: {answer}");
}

fn parse_line(line: &str) -> impl Iterator<Item = String> {
	assert!(line.len() % 2 == 0);

	let bytes = (0..line.len())
		.step_by(2)
		.map(|i| &line[i..(i + 2)])
		.map(|chunk| u8::from_str_radix(chunk, 16))
		.collect::<Result<Vec<_>, _>>()
		.unwrap();

	[
		parse_str_utf8(&bytes),
		Some(parse_str_latin1(&bytes)),
		parse_str_utf16le(&bytes),
		parse_str_utf16be(&bytes),
	]
	.into_iter()
	.flatten()
}

fn parse_str_utf8(bytes: &[u8]) -> Option<String> {
	let str = match bytes {
		[0xEF, 0xBB, 0xBF, str @ ..] => str, /* Trim BOM */
		str => str,
	};
	str::from_utf8(str).map(String::from).ok()
}

fn parse_str_latin1(bytes: &[u8]) -> String {
	bytes.iter().map(|b| *b as char).collect()
}

fn parse_str_utf16le(bytes: &[u8]) -> Option<String> {
	fn parse_utf16(str: &[u16]) -> Result<String, DecodeUtf16Error> {
		decode_utf16(str.iter().cloned()).collect()
	}

	if bytes.is_empty() || bytes.len() % 2 != 0 {
		return None;
	}

	match unsafe { bytes.align_to::<u16>() }.1 {
		[0xFEFF, u16s @ ..] => {
			let str = parse_utf16(u16s).unwrap();
			Some(str)
		}
		[0xFFFE, ..] => None,
		other => parse_utf16(other).ok(),
	}
}

fn parse_str_utf16be(bytes: &[u8]) -> Option<String> {
	if bytes.len() % 2 != 0 {
		return None;
	}

	let bytes = bytes
		.chunks(2)
		.flat_map(|chunk| [chunk[1], chunk[0]])
		.collect::<Vec<_>>();
	parse_str_utf16le(&bytes)
}
