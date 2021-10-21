use std::char::{ToLowercase, ToUppercase};

use crate::Transform;

/// Lower-case transform.
pub fn lower() -> Lower {
	Lower {}
}

/// Upper-case transform.
pub fn upper() -> Upper {
	Upper {}
}

/// Transform implementation for [`lower`].
pub struct Lower {}

impl<I: Iterator<Item = char>> Transform<I> for Lower {
	type Output = LowerIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		LowerIter {
			input: input,
			lower: None,
		}
	}
}

/// Transform implementation for [`upper`].
pub struct Upper {}

impl<I: Iterator<Item = char>> Transform<I> for Upper {
	type Output = UpperIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		UpperIter {
			input: input,
			upper: None,
		}
	}
}

/// Transform iterator for [`lower`].
pub struct LowerIter<I: Iterator<Item = char>> {
	input: I,
	lower: Option<ToLowercase>,
}

impl<I: Iterator<Item = char>> Iterator for LowerIter<I> {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		loop {
			if let Some(lower) = &mut self.lower {
				if let Some(next) = lower.next() {
					return Some(next);
				} else {
					self.lower = None;
				}
			}

			match self.input.next() {
				Some(char) => {
					self.lower = Some(char.to_lowercase());
				}
				None => {
					return None;
				}
			}
		}
	}
}

/// Transform iterator for [`upper`].
pub struct UpperIter<I: Iterator<Item = char>> {
	input: I,
	upper: Option<ToUppercase>,
}

impl<I: Iterator<Item = char>> Iterator for UpperIter<I> {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		loop {
			if let Some(upper) = &mut self.upper {
				if let Some(next) = upper.next() {
					return Some(next);
				} else {
					self.upper = None;
				}
			}

			match self.input.next() {
				Some(char) => {
					self.upper = Some(char.to_uppercase());
				}
				None => {
					return None;
				}
			}
		}
	}
}
