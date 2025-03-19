#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrosswordEntry {
	word_len: usize,
	ch_index: usize,
	ch: char,
}

impl CrosswordEntry {
	pub fn matches(&self, word: &str) -> bool {
		word.chars().count() == self.word_len
			&& word
				.chars()
				.nth(self.ch_index)
				.is_some_and(|ch| ch == self.ch)
	}
}

pub fn parse_crossword(
	lines: impl Iterator<Item = impl AsRef<str>>,
) -> impl Iterator<Item = CrosswordEntry> {
	lines.map(|line| {
		let line = line.as_ref().trim();
		let word_len = line.chars().count();
		let (ch_index, ch) = line
			.trim()
			.char_indices()
			.find(|(_, ch)| *ch != '.')
			.unwrap();

		CrosswordEntry {
			word_len,
			ch_index,
			ch,
		}
	})
}
