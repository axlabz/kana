use std::char::{ToLowercase, ToUppercase};

use crate::{Text, Transform};

/// Lower-case the input text.
pub fn lower() -> Lower {
	Lower {}
}

/// Upper-case the input text.
pub fn upper() -> Upper {
	Upper {}
}

pub struct Lower {}

impl<I: Iterator<Item = Text>> Transform<I> for Lower {
	type Output = LowerIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		LowerIter {
			inner: input,
			lower: None,
			after: None,
		}
	}
}

pub struct Upper {}

impl<I: Iterator<Item = Text>> Transform<I> for Upper {
	type Output = UpperIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		UpperIter {
			inner: input,
			upper: None,
			after: None,
		}
	}
}

pub struct LowerIter<I: Iterator<Item = Text>> {
	inner: I,
	lower: Option<ToLowercase>,
	after: Option<&'static str>,
}

impl<I: Iterator<Item = Text>> Iterator for LowerIter<I> {
	type Item = Text;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(lower) = &mut self.lower {
				if let Some(next) = lower.next() {
					return Some(Text::Char(next));
				} else {
					self.lower = None;
				}
			}

			if let Some(str) = self.after {
				let next = str.chars().next().unwrap();
				self.lower = Some(next.to_lowercase());
				let str = &str[next.len_utf8()..];
				self.after = if str.len() > 0 { Some(str) } else { None };
			} else {
				match self.inner.next() {
					None => {
						return None;
					}
					Some(Text::Char(char)) => {
						self.lower = Some(char.to_lowercase());
					}
					Some(Text::Static(str)) => {
						self.after = Some(str);
					}
				}
			}
		}
	}
}

pub struct UpperIter<I: Iterator<Item = Text>> {
	inner: I,
	upper: Option<ToUppercase>,
	after: Option<&'static str>,
}

impl<I: Iterator<Item = Text>> Iterator for UpperIter<I> {
	type Item = Text;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if let Some(upper) = &mut self.upper {
				if let Some(next) = upper.next() {
					return Some(Text::Char(next));
				} else {
					self.upper = None;
				}
			}

			if let Some(str) = self.after {
				let next = str.chars().next().unwrap();
				self.upper = Some(next.to_uppercase());
				let str = &str[next.len_utf8()..];
				self.after = if str.len() > 0 { Some(str) } else { None };
			} else {
				match self.inner.next() {
					None => {
						return None;
					}
					Some(Text::Char(char)) => {
						self.upper = Some(char.to_uppercase());
					}
					Some(Text::Static(str)) => {
						self.after = Some(str);
					}
				}
			}
		}
	}
}
