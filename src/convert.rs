//! Generic text conversion support.

/// Returns a [`char`] iterator that applies the text [`Transform`] to the
/// input iterator.
pub fn convert<T: Transform<I::IntoIter>, I: IntoIterator<Item = char>>(
	transform: T,
	input: I,
) -> impl Iterator<Item = char> {
	let input = input.into_iter();
	transform.convert(input)
}

//============================================================================//
// Transform
//============================================================================//

/// Trait implemented for generic text transforms used by [`convert`].
///
/// A text transform is basically a factory that consumes an [`Iterator`] as
/// input and returns a new iterator that outputs the transform result to it.
pub trait Transform<I: Iterator<Item = char>> {
	/// Type of the output [`char`] iterator.
	type Output: Iterator<Item = char>;

	/// Consumes the input iterator and returns the transformed output
	/// iterator.
	fn convert(&self, input: I) -> Self::Output;
}
