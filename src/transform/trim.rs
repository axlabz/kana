use crate::{is_space, Chars, Transform};

/// Removes leading and trailing space, normalizes multiple spaces to a single
/// U+0020 space character.
pub fn trim() -> impl Transform {
	Trim {
		start: true,
		space: false,
	}
}

struct Trim {
	start: bool,
	space: bool,
}

impl Transform for Trim {
	fn push(&mut self, next: char) -> Chars {
		if is_space(next) {
			if !self.start {
				self.space = true;
			}
			Chars::None
		} else {
			self.start = false;
			if self.space {
				self.space = false;
				Chars::L2([' ', next], false)
			} else {
				Chars::L1([next], false)
			}
		}
	}
}
