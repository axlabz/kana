use crate::{is_space, Text, Transform};

/// Removes leading and trailing space, normalizes multiple spaces to a single
/// U+0020 space character.
pub fn trim() -> Trim {
	Trim {}
}

pub struct Trim {}

impl<I: Iterator<Item = Text>> Transform<I> for Trim {
	type Output = TrimIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		TrimIter {
			inner: input,
			start: true,
			space: false,
			next: None,
		}
	}
}

pub struct TrimIter<I: Iterator<Item = Text>> {
	inner: I,
	start: bool,
	space: bool,
	next: Option<Text>,
}

impl<I: Iterator<Item = Text>> Iterator for TrimIter<I> {
	type Item = Text;

	fn next(&mut self) -> Option<Self::Item> {
		if let output @ Some(_) = std::mem::take(&mut self.next) {
			return output;
		}

		loop {
			match self.inner.next() {
				None => {
					return None;
				}
				Some(Text::Char(char)) => {
					if is_space(char) {
						if !self.start {
							self.space = true;
						}
					} else {
						self.start = false;
						if self.space {
							self.space = false;
							self.next = Some(Text::Char(char));
							return Some(Text::Char(' '));
						} else {
							return Some(Text::Char(char));
						}
					}
				}
				Some(Text::Static(str)) => {
					if str.len() > 0 {
						let trim1 = str.trim_start_matches(is_space);
						if trim1.len() == 0 {
							if !self.start {
								self.space = true;
							}
						} else {
							if trim1.len() != str.len() {
								if !self.start {
									self.space = true;
								}
							}

							self.start = false;

							let trim2 = trim1.trim_end_matches(is_space);
							let space_end = if trim2.len() != trim1.len() {
								true
							} else {
								false
							};

							if self.space {
								self.space = space_end;
								self.next = Some(Text::Static(str));
								return Some(Text::Char(' '));
							} else {
								self.space = space_end;
								return Some(Text::Static(trim2));
							}
						}
					}
				}
			}
		}
	}
}
