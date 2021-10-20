use std::marker::PhantomData;

use crate::{Text, Transform};

pub trait Chainable<I: Iterator<Item = Text>, A: Transform<I>, B: Transform<A::Output>> {
	fn chain(self, other: B) -> ChainTransform<I, A, B>;
}

impl<I: Iterator<Item = Text>, A: Transform<I>, B: Transform<A::Output>> Chainable<I, A, B> for A {
	/// Returns a new transform that applies the given `other` transform to
	/// the output of the current transform.
	fn chain(self, other: B) -> ChainTransform<I, A, B> {
		ChainTransform::new(self, other)
	}
}

pub struct ChainTransform<I: Iterator<Item = Text>, A: Transform<I>, B: Transform<A::Output>> {
	head: A,
	tail: B,
	phantom: PhantomData<I>,
}

impl<I: Iterator<Item = Text>, A: Transform<I>, B: Transform<A::Output>> ChainTransform<I, A, B> {
	fn new(head: A, tail: B) -> Self {
		Self {
			head,
			tail,
			phantom: PhantomData,
		}
	}
}

impl<I: Iterator<Item = Text>, A: Transform<I>, B: Transform<A::Output>> Transform<I>
	for ChainTransform<I, A, B>
{
	type Output = B::Output;

	fn convert(&self, input: I) -> Self::Output {
		self.tail.convert(self.head.convert(input))
	}
}
