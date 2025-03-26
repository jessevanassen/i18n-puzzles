use core::str;
use std::{fmt::Debug, io::stdin};

use i18n_puzzles::utf8;
use itertools::Itertools;

const TOP_LEFT: [u8; 3] = [0xe2, 0x95, 0x94];
const BOTTOM_LEFT: [u8; 3] = [0xe2, 0x95, 0x9a];

type MapKeys = Vec<Vec<(u32, u32)>>;
fn main() {
	let map_pieces = parse_input();
	let horizontal_block_size = map_pieces.first().map(|piece| piece.width()).unwrap();
	let vertical_block_size = map_pieces.iter().map(|piece| piece.height()).min().unwrap();

	for (i, piece) in map_pieces.iter().enumerate() {
		println!("Fragment {i}:");
		print_piece(piece);
		println!();
	}

	let map_keys: MapKeys = map_pieces
		.iter()
		.map(|piece| {
			(0..(piece.height() / vertical_block_size))
				.map(|i| piece.keys(i, vertical_block_size))
				.collect()
		})
		.collect();

	for (i, key) in map_keys.iter().enumerate() {
		println!("Fragment {i}:");
		for k in key {
			println!("{0:02x} ({0:08b}), {1:02x} ({1:08b})", k.0, k.1);
		}
		println!()
	}

	let map_width = map_pieces.iter().filter(|piece| piece.is_top()).count() + 2;

	let map_height = map_pieces
		.iter()
		.filter(|piece| piece.is_top_left() || piece.is_left() || piece.is_bottom_left())
		.map(|piece| piece.height())
		.sum::<usize>()
		/ vertical_block_size;

	let mut filled = FilledPieces::new(map_width, map_height);

	{
		let (top_left_index, top_left) = map_pieces
			.iter()
			.enumerate()
			.find(|(_, piece)| piece.is_top_left())
			.unwrap();
		filled.set(
			0,
			0,
			top_left_index,
			top_left.height() / vertical_block_size,
		);
	}

	// {
	// 	let (top_right_index, top_right) = map_pieces
	// 		.iter()
	// 		.enumerate()
	// 		.find(|(_, piece)| piece.is_top_right())
	// 		.unwrap();
	// 	dbg!(
	// 		0,
	// 		filled.width - 1,
	// 		top_right_index,
	// 		top_right.height() / vertical_block_size,
	// 	);
	// 	filled.set(
	// 		0,
	// 		filled.width - 1,
	// 		top_right_index,
	// 		top_right.height() / vertical_block_size,
	// 	);
	// }

	// {
	// 	let (bottom_left_index, bottom_left) = map_pieces
	// 		.iter()
	// 		.enumerate()
	// 		.find(|(_, piece)| piece.is_bottom_left())
	// 		.unwrap();
	// 	filled.set(
	// 		filled.height - bottom_left.height() / vertical_block_size,
	// 		0,
	// 		bottom_left_index,
	// 		bottom_left.height() / vertical_block_size,
	// 	);
	// }

	// {
	// 	let (bottom_right_index, bottom_right) = map_pieces
	// 		.iter()
	// 		.enumerate()
	// 		.find(|(_, piece)| piece.is_bottom_right())
	// 		.unwrap();
	// 	filled.set(
	// 		filled.height - bottom_right.height() / vertical_block_size,
	// 		filled.width - 1,
	// 		bottom_right_index,
	// 		bottom_right.height() / vertical_block_size,
	// 	);
	// }

	filled.solve(&map_keys);

	let mut map: Vec<Vec<u8>> = vec![vec![0; map_width * horizontal_block_size]; map_height * vertical_block_size];
	for (r, c) in filled.indices() {
		let (piece_index, piece_index_offset) = filled.key_index(r, c).unwrap();
		let piece = &map_pieces[piece_index];

		for r2 in 0..vertical_block_size {
			for c2 in 0..horizontal_block_size {
				let b = piece.bytes[r2 + piece_index_offset * vertical_block_size][c2];
				map[r * vertical_block_size + r2][c * horizontal_block_size + c2] = b;
			}
		}
	}

	for (r, line) in map.into_iter().enumerate() {
		let line = match str::from_utf8(&line) {
			Ok(str) => str,
			Err(err) => str::from_utf8(&line[..err.valid_up_to()]).unwrap(),
		};
		println!("{line}");

		if let Some(c) = line.chars().position(|ch| ch == '╳') {
			dbg!(r, c, r * c);
		}
	}

	// let (key, (r, c)) = map_pieces
	// 	.iter()
	// 	.enumerate()
	// 	.find_map(|(i, piece)| piece.find_x().map(|c| (i, c)))
	// 	.unwrap();

	// let (r2, c2) = filled
	// 	.indices()
	// 	.find(|(r, c)| filled.get(*r, *c).unwrap() == key)
	// 	.unwrap();

	// dbg!((r2 * vertical_block_size + r) * (c2 * horizontal_block_size + c));
}

fn print_piece(piece: &MapPiece) {
	fn print_row(row: &[u8]) {
		match str::from_utf8(row) {
			Ok(str) => println!("{str}"),
			Err(err) if err.valid_up_to() > 0 => {
				let str = str::from_utf8(&row[..(err.valid_up_to())]).unwrap();
				println!("{str}�");
			}
			Err(_) => {
				let first_starting_character = row
					.iter()
					.position(|b| !utf8::is_continuation_byte(*b))
					.unwrap_or(row.len());
				// print!("�");
				print_row(&row[first_starting_character..]);
			}
		}
	}

	for row in piece.bytes.iter() {
		print_row(row);
	}
}

struct MapPiece {
	bytes: Vec<Vec<u8>>,
}

impl MapPiece {
	fn keys(&self, block_index: usize, block_size: usize) -> (u32, u32) {
		fn key(
			piece: &MapPiece,
			block_index: usize,
			block_size: usize,
			key_fn: fn(&Vec<u8>) -> usize,
		) -> u32 {
			piece
				.bytes
				.iter()
				.skip(block_index * block_size)
				.take(block_size)
				.map(key_fn)
				.enumerate()
				.fold(0, |acc, (i, c)| acc | ((c & 0b11) << (i * 2)) as u32)
		}

		(
			key(self, block_index, block_size, |line| {
				line.iter()
					.take_while(|&&b| utf8::is_continuation_byte(b))
					.count()
			}),
			key(self, block_index, block_size, |line| {
				let (i, b) = line
					.iter()
					.rev()
					.find_position(|&&b| !utf8::is_continuation_byte(b))
					.unwrap();
				utf8::sequence_size(*b).unwrap() as usize - 1 - i
			}),
		)
	}

	fn is_top_left(&self) -> bool {
		self.bytes
			.first()
			.map(|row| row.windows(TOP_LEFT.len()).next() == Some(&TOP_LEFT))
			.unwrap_or(false)
	}

	fn is_bottom_left(&self) -> bool {
		self.bytes
			.last()
			.map(|row| row.windows(BOTTOM_LEFT.len()).next() == Some(&BOTTOM_LEFT))
			.unwrap_or(false)
	}

	fn is_top(&self) -> bool {
		self.bytes
			.first()
			.and_then(|row| str::from_utf8(row).ok())
			.map(|row| row.chars().all(|ch| matches!(ch, '-' | '═')))
			.unwrap_or(false)
	}

	fn is_left(&self) -> bool {
		self.bytes
			.iter()
			.all(|row| matches!(&row[..], &[b'|', ..] | &[0xe2, 0x95, 0x91, ..]))
	}

	fn width(&self) -> usize {
		self.bytes.first().map(|row| row.len()).unwrap_or(0)
	}

	fn height(&self) -> usize {
		self.bytes.len()
	}
}

pub struct FilledPieces {
	filled: Vec<Option<usize>>,
	width: usize,
	height: usize,
}

impl FilledPieces {
	pub fn new(width: usize, height: usize) -> Self {
		FilledPieces {
			filled: vec![None; width * height],
			width,
			height,
		}
	}

	pub fn get(&self, row: usize, column: usize) -> Option<usize> {
		self.filled[self.idx(row, column)?]
	}

	pub fn set(&mut self, row: usize, column: usize, index: usize, height: usize) -> bool {
		if !self.in_range(row + height - 1, column) {
			return false;
		}

		for r in row..(row + height) {
			let idx = self.idx(r, column).unwrap();
			self.filled[idx] = Some(index);
		}

		true
	}

	fn in_range(&self, row: usize, column: usize) -> bool {
		row < self.height && column < self.width
	}

	fn idx(&self, row: usize, column: usize) -> Option<usize> {
		self.in_range(row, column)
			.then_some(row * self.width + column)
	}

	pub fn space_for(&self, row: usize, column: usize) -> usize {
		(row..self.height)
			.take_while(|r| self.get(*r, column).is_none())
			.count()
	}

	pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
		(0..self.height).flat_map(|r| (0..self.width).map(move |c| (r, c)))
	}

	pub fn key_index(&self, row: usize, column: usize) -> Option<(usize, usize)> {
		let v = self.get(row, column)?;
		Some((
			v,
			(0..row)
				.rev()
				.take_while(|r| self.get(*r, column) == Some(v))
				.count(),
		))
	}

	pub fn solve(&mut self, keys: &MapKeys) -> bool {
		fn key_used(filled_pieces: &FilledPieces, index: usize) -> bool {
			filled_pieces.filled.iter().contains(&Some(index))
		}

		'outer: loop {
			if self.filled.iter().all(|x| x.is_some()) {
				return true;
			}

			for (row, column) in self.indices().collect::<Vec<_>>() {
				// Uggh
				for (key_index, key) in keys.iter().enumerate() {
					if self.space_for(row, column) < key.len() || key_used(self, key_index) {
						continue;
					}

					let fit_left = if column > 0 {
						(row..row + key.len()).enumerate().any(|(i, r)| {
							self.key_index(r, column - 1)
								.is_some_and(|(k, ki)| keys[k][ki].1 == key[i].0)
						})
					} else {
						false
					};

					let fit_right = if column < self.width - 1 {
						(row..row + key.len()).enumerate().any(|(i, r)| {
							self.key_index(r, column + 1)
								.is_some_and(|(k, ki)| keys[k][ki].0 == key[i].1)
						})
					} else {
						false
					};

					if fit_left || fit_right {
						self.set(row, column, key_index, key.len());
						continue 'outer;
					}
				}
			}

			return false;
		}
	}
}

impl Debug for FilledPieces {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}x{}", self.width, self.height)?;
		for r in 0..self.height {
			for c in 0..self.width {
				if let Some(i) = self.get(r, c) {
					write!(f, "{i:4} ")?;
				} else {
					write!(f, "     ")?;
				}
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

fn parse_input() -> Vec<MapPiece> {
	let mut bytes = vec![vec![]];

	for line in stdin().lines().map(Result::unwrap) {
		if line.is_empty() {
			bytes.push(vec![]);
		} else {
			bytes.last_mut().unwrap().push(parse_bytes(&line));
		}
	}

	bytes.into_iter().map(|bytes| MapPiece { bytes }).collect()
}

fn parse_bytes(str: &str) -> Vec<u8> {
	assert!(str.len() % 2 == 0);

	(0..str.len())
		.step_by(2)
		.map(|i| &str[i..(i + 2)])
		.map(|b| u8::from_str_radix(b, 16).unwrap())
		.collect()
}
