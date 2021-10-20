//! Generic text conversion support.

use std::iter::FromIterator;

/// Returns a [`char`] iterator that applies the text [`Transform`] to the
/// input iterator.
pub fn convert<T: Transform<CharIterator<I::IntoIter>>, I: IntoIterator<Item = char>>(
	transform: T,
	input: I,
) -> impl Iterator<Item = Text> {
	let input = CharIterator::new(input.into_iter());
	transform.convert(input)
}

pub struct CharIterator<I: Iterator<Item = char>> {
	iter: I,
}

impl<I: Iterator<Item = char>> CharIterator<I> {
	fn new(iter: I) -> Self {
		CharIterator { iter }
	}
}

impl<I: Iterator<Item = char>> Iterator for CharIterator<I> {
	type Item = Text;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|x| x.into())
	}
}

impl FromIterator<Text> for String {
	fn from_iter<T: IntoIterator<Item = Text>>(iter: T) -> Self {
		let mut out = String::new();
		for it in iter {
			match it {
				Text::Char(char) => out.push(char),
				Text::Static(str) => out.push_str(str),
			}
		}
		out
	}
}

//============================================================================//
// Transform
//============================================================================//

/// Trait implemented for generic text transforms used by [`convert`].
pub trait Transform<I: Iterator<Item = Text>> {
	type Output: Iterator<Item = Text>;

	fn convert(&self, input: I) -> Self::Output;
}

//============================================================================//
// CharResult & Chars
//============================================================================//

#[derive(Debug)]
pub enum Text {
	Char(char),
	Static(&'static str),
}

impl From<char> for Text {
	fn from(c: char) -> Self {
		Text::Char(c)
	}
}
