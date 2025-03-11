use std::io::stdin;

const PILE_OF_POO: char = '💩';

fn main() {
	let lines = stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

	let line_length = lines[0].chars().count();

	let times_stepped_in_poo = lines
		.iter()
		.enumerate()
		.map(|(index, line)| {
			let char_index = index * 2 % line_length;
			line.chars().nth(char_index).unwrap()
		})
		.filter(|&char| char == PILE_OF_POO)
		.count();

	println!("Stepped in poo {times_stepped_in_poo} times");
}
