use std::io::stdin;

const ODYSSEUS_VARIANTS: [&str; 5] = ["Οδυσσευς", "Οδυσσεως", "Οδυσσει", "Οδυσσεα", "Οδυσσευ"];

const UPPERCASE_ALPHA: char = 'Α';
const LOWERCASE_ALPHA: char = 'α';
const UPPERCASE_RHO: char = 'Ρ';
const LOWERCASE_RHO: char = 'ρ';
const UPPERCASE_SIGMA: char = 'Σ';
const LOWERCASE_SIGMA: char = 'σ';
const UPPERCASE_OMEGA: char = 'Ω';
const LOWERCASE_OMEGA: char = 'ω';

fn main() {
	let answer = stdin()
		.lines()
		.map(Result::unwrap)
		.filter_map(|mut line| {
			/* Not rotated inputs can be ignored, as those would result in '0'
			 * which will be ignored when summing. */

			for i in 1..=23 {
				line = rot_str(&line);

				if ODYSSEUS_VARIANTS
					.iter()
					.any(|odysseus| line.contains(odysseus))
				{
					return Some(i);
				}
			}

			None
		})
		.sum::<u32>();

	println!("Answer: {answer}");
}

fn rot_str(input: &str) -> String {
	input.chars().map(rot_char).collect()
}

fn rot_char(ch: char) -> char {
	match ch {
		UPPERCASE_OMEGA => UPPERCASE_ALPHA,
		LOWERCASE_OMEGA => LOWERCASE_ALPHA,
		UPPERCASE_RHO => UPPERCASE_SIGMA,
		LOWERCASE_RHO => LOWERCASE_SIGMA,
		ch if is_greek_letter(ch) => char::try_from(u32::from(ch) + 1).unwrap(),
		ch => ch,
	}
}

fn is_greek_letter(ch: char) -> bool {
	(UPPERCASE_ALPHA..=UPPERCASE_OMEGA).contains(&ch)
		|| (LOWERCASE_ALPHA..=LOWERCASE_OMEGA).contains(&ch)
}
