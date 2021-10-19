//! Generic text conversion support.

/// Returns a [`char`] iterator that applies the text [`Transform`] to the
/// input iterator.
pub fn convert<T: Transform, I: IntoIterator<Item = char>>(
	transform: T,
	input: I,
) -> impl Iterator<Item = char> {
	let input = input.into_iter();
	ConvertIterator::new(transform, input)
}

/// Iterator returned by [`convert`].
pub struct ConvertIterator<T: Transform, I: Iterator<Item = char>> {
	/// The transform we are applying.
	transform: T,
	/// The source [`char`] iterator.
	iter: I,
	/// Pending output from the transform.
	chars: Chars,
	/// True if the transform has more characters to be drained.
	more: bool,
	/// True if we reached the end of the input iterator.
	done: bool,
}

impl<T: Transform, I: Iterator<Item = char>> ConvertIterator<T, I> {
	fn new(transform: T, iter: I) -> Self {
		ConvertIterator {
			transform,
			iter,
			chars: Chars::default(),
			more: false,
			done: false,
		}
	}
}

impl<T: Transform, I: Iterator<Item = char>> Iterator for ConvertIterator<T, I> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		// loop because a transform may take any number of input characters
		// before outputting a char
		loop {
			match self.chars {
				// if there is any pending output in `self.chars`, consume the
				// first char and rotate the value
				Chars::L1([a], more) => {
					self.chars = Chars::None;
					self.more = more; // do we need to call `drain` next time?
					return Some(a);
				}
				Chars::L2([a, b], more) => {
					self.chars = Chars::L1([b], more);
					return Some(a);
				}
				Chars::L3([a, b, c], more) => {
					self.chars = Chars::L2([b, c], more);
					return Some(a);
				}
				Chars::L4([a, b, c, d], more) => {
					self.chars = Chars::L3([b, c, d], more);
					return Some(a);
				}

				// no output readily available, take it from the transform
				Chars::None => {
					// first check if we have pending output from the previous
					// `push` or `finish`
					if self.more {
						self.more = false;
						self.chars = self.transform.drain();
					} else if self.done {
						// if the input iterator is done, then we are done
						return None;
					} else {
						// consume the next input character from the iterator
						if let Some(next) = self.iter.next() {
							self.chars = self.transform.push(next);
						} else {
							// the iterator is done, call finish on the
							// transform and consume any final output
							self.done = true;
							self.chars = self.transform.finish();
						}
					}
				}
			}
		}
	}
}

//============================================================================//
// Transform
//============================================================================//

/// Trait implemented for generic text transforms used by [`convert`].
pub trait Transform {
	/// Pushes a new character onto the transform. Returns the transformed
	/// text as a [`Chars`] value.
	///
	/// If the returned value has the `more` flag set (see [`Chars::has_more`])
	/// then the additional output MUST be consumed by [`drain`](Transform::drain)
	/// before calling `next` again.
	///
	/// The returned value may also be [`Chars::None`], in which case the
	/// transform either maps to the empty string, or the transform needs
	/// more lookahead context to determine the output.
	///
	/// After all input characters have been pushed and the output drained,
	/// the [`finish`](Transform::finish) method must be called to flag the
	/// end of the input and retrieve any remaining output.
	fn push(&mut self, next: char) -> Chars;

	/// Drains additional output after a call to [`push`](Transform::push)
	/// or [`finish`](Transform::finish) returns a partial value.
	///
	/// If there is nothing to drain, this returns [`Chars::None`]. This is
	/// safe to call at any point during the transform.
	fn drain(&mut self) -> Chars {
		Chars::None
	}

	/// Flags the end of input to the transform and returns any final output.
	///
	/// This should only be called after [draining](Transform::drain) all
	/// output from the last [`push`](Transform::push) call.
	///
	/// Note that like `push`, this can return a partial output, in which case
	/// [`drain`](Transform::drain) must be called to retrieve all output.
	fn finish(&mut self) -> Chars {
		Chars::None
	}
}

//============================================================================//
// CharResult & Chars
//============================================================================//

/// Value type for returning small lists of up to four characters.
///
/// The purpose of this type is to provide an allocation-free value type to
/// return a variable-length but small list of characters.
///
/// This is used by the [`Transform`] trait, for which the expected common
/// case is returning a few characters for each input character.
///
/// ## More characters (chunks)
///
/// This type can return at most four characters at a time. Anything above
/// that length must be returned in chunks.
///
/// For that purpose, the non-empty values of [`Chars`] provide a final `more`
/// flag as a boolean. If this is true, it indicates the current value is
/// partial and more data MAY be available.
///
/// Note that "may" is important. It is perfectly valid to return a partial
/// result and then return [`Chars::None`] on the next chunk.
#[derive(Debug, Copy, Clone)]
pub enum Chars {
	None,
	/// Single character.
	L1([char; 1], bool),
	/// Pair of characters.
	L2([char; 2], bool),
	/// Triple of characters.
	L3([char; 3], bool),
	/// Tuple of four characters.
	L4([char; 4], bool),
}

impl From<char> for Chars {
	fn from(c: char) -> Self {
		Chars::L1([c], false)
	}
}

impl Default for Chars {
	fn default() -> Self {
		Chars::None
	}
}

impl Chars {
	/// Consumes the beginning of the iterator returning a [`Chars`] value
	///
	/// This will try to read as many characters as can be returned by a
	/// single [`Chars`] and will set the `more` flag if the iterator has
	/// not reached the end.
	pub fn read_iter<T: Iterator<Item = char>>(iter: &mut T) -> Chars {
		match iter.next() {
			None => Chars::None,
			Some(a) => match iter.next() {
				None => Chars::L1([a], false),
				Some(b) => match iter.next() {
					None => Chars::L2([a, b], false),
					Some(c) => match iter.next() {
						None => Chars::L3([a, b, c], false),
						Some(d) => Chars::L4([a, b, c, d], true),
					},
				},
			},
		}
	}

	/// Returns the list of characters.
	pub fn list(&self) -> Option<&[char]> {
		match self {
			Chars::L1(list, _) => Some(list),
			Chars::L2(list, _) => Some(list),
			Chars::L3(list, _) => Some(list),
			Chars::L4(list, _) => Some(list),
			Chars::None => None,
		}
	}

	/// Returns `true` for [`Chars::None`].
	pub fn is_none(&self) -> bool {
		if let Chars::None = self {
			true
		} else {
			false
		}
	}

	/// Returns `true` if the `more` flag is set.
	///
	/// This flag being set indicates that the current value is partial and
	/// more data could be available, which must be retrieved from the source.
	pub fn has_more(&self) -> bool {
		match self {
			&Chars::L1(_, more) => more,
			&Chars::L2(_, more) => more,
			&Chars::L3(_, more) => more,
			&Chars::L4(_, more) => more,
			Chars::None => false,
		}
	}
}
