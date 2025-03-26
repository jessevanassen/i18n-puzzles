pub fn is_starting_byte(b: u8) -> bool {
	b > 127 && !is_continuation_byte(b)
}

pub fn is_continuation_byte(b: u8) -> bool {
	(b & 0b1100_0000) == 0b1000_0000
}

pub fn sequence_size(b: u8) -> Option<u8> {
	match b {
		b if b & 0b1000_0000 == 0 => Some(1),
		b if b & 0b1110_0000 == 0b1100_0000 => Some(2),
		b if b & 0b1111_0000 == 0b1110_0000 => Some(3),
		b if b & 0b1111_1000 == 0b1111_0000 => Some(4),
		_ => None,
	}
}
