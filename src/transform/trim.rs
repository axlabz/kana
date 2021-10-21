use crate::{is_space, Transform};

/// Removes leading and trailing space, normalizes multiple spaces to a single
/// U+0020 space character.
///
/// This uses [`is_space`] to test for space characters, which includes line
/// breaks.
pub fn trim() -> Trim {
	Trim {}
}

/// Transform implementation for [`trim`].
pub struct Trim {}

impl<I: Iterator<Item = char>> Transform<I> for Trim {
	type Output = TrimIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		TrimIter {
			input: input,
			start: true,
			space: false,
			ahead: None,
		}
	}
}

/// Transform iterator for [`trim`].
pub struct TrimIter<I: Iterator<Item = char>> {
	input: I,
	start: bool,
	space: bool,
	ahead: Option<char>,
}

impl<I: Iterator<Item = char>> Iterator for TrimIter<I> {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		if let Some(char) = std::mem::take(&mut self.ahead) {
			return Some(char);
		}

		loop {
			match self.input.next() {
				None => return None,
				Some(char) => {
					if is_space(char) {
						if !self.start {
							self.space = true;
						}
					} else {
						self.start = false;
						if self.space {
							self.space = false;
							self.ahead = Some(char);
							return Some(' ');
						} else {
							return Some(char);
						}
					}
				}
			}
		}
	}
}
