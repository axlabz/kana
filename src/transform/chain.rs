use std::marker::PhantomData;

use crate::Transform;

/// Extension method to chain two transforms using [`ChainTransform`].
pub trait Chainable<I: Iterator<Item = char>, A: Transform<I>, B: Transform<A::Output>> {
	/// Returns a new transform that applies the `other` transform to the
	/// output of the current transform.
	fn chain(self, other: B) -> ChainTransform<I, A, B>;
}

impl<I: Iterator<Item = char>, A: Transform<I>, B: Transform<A::Output>> Chainable<I, A, B> for A {
	/// Returns a new transform that applies the given `other` transform to
	/// the output of the current transform.
	fn chain(self, other: B) -> ChainTransform<I, A, B> {
		ChainTransform::new(self, other)
	}
}

/// Transform that chains two transforms together, applying the second one to
/// the output of the first.
pub struct ChainTransform<I: Iterator<Item = char>, A: Transform<I>, B: Transform<A::Output>> {
	head: A,
	tail: B,
	phantom: PhantomData<I>,
}

impl<I: Iterator<Item = char>, A: Transform<I>, B: Transform<A::Output>> ChainTransform<I, A, B> {
	fn new(head: A, tail: B) -> Self {
		Self {
			head,
			tail,
			phantom: PhantomData,
		}
	}
}

impl<I: Iterator<Item = char>, A: Transform<I>, B: Transform<A::Output>> Transform<I>
	for ChainTransform<I, A, B>
{
	type Output = B::Output;

	fn convert(&self, input: I) -> Self::Output {
		self.tail.convert(self.head.convert(input))
	}
}
