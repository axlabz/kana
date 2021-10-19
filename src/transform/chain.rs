use crate::{Chars, Transform};

pub trait Chainable<A: Transform, B: Transform> {
	fn chain(self, other: B) -> Chain<A, B>;
}

impl<A: Transform, B: Transform> Chainable<A, B> for A {
	/// Returns a new transform that applies the given `other` transform to
	/// the output of the current transform.
	fn chain(self, other: B) -> Chain<A, B> {
		Chain::new(self, other)
	}
}

pub struct Chain<A: Transform, B: Transform> {
	/// First transform.
	head: A,
	/// Chained transform.
	tail: B,

	/// Pending input from the `head` transformation.
	pending: Chars,

	/// Set to true if `finish` has been called but we didn't complete the
	/// output.
	finishing: bool,
}

impl<A: Transform, B: Transform> Transform for Chain<A, B> {
	fn push(&mut self, next: char) -> Chars {
		// push the character to the `head` iterator
		self.pending = self.head.push(next);
		// take the output of the head and forward to `tail`
		self.take_next()
	}

	fn drain(&mut self) -> Chars {
		// first we try to drain any pending output from `tail`
		match self.tail.drain() {
			// if there is no pending output, we process the next pending
			// character from `head` (if any)
			Chars::None => self.take_next(),
			output => output,
		}
	}

	fn finish(&mut self) -> Chars {
		// first we call finish on the `head` transform
		self.finishing = true;
		self.pending = self.head.finish();
		// we need to forward any final output from `head`
		self.take_next()
	}
}

impl<A: Transform, B: Transform> Chain<A, B> {
	fn new(a: A, b: B) -> Chain<A, B> {
		Chain {
			head: a,
			tail: b,
			pending: Chars::default(),
			finishing: false,
		}
	}

	/// Takes the next character from the `pending` output from the `head`
	/// transform and pushes it to `tail`.
	fn take_next(&mut self) -> Chars {
		// take the next character and compute the new `pending` value
		let (pending, next) = match self.pending {
			Chars::None => (Chars::None, None),
			Chars::L1([a], more) => {
				// for the last character, if there is more output we drain it
				// from `head`
				if more {
					(self.head.drain(), Some(a))
				} else {
					(Chars::None, Some(a))
				}
			}
			// for two or more character we just take the first character and
			// cycle the rest
			Chars::L2([a, b], more) => (Chars::L1([b], more), Some(a)),
			Chars::L3([a, b, c], more) => (Chars::L2([b, c], more), Some(a)),
			Chars::L4([a, b, c, d], more) => (Chars::L3([b, c, d], more), Some(a)),
		};

		// update the pending value and push the next character (if any)
		// to the `tail` transform
		self.pending = pending;
		if let Some(next) = next {
			self.tail.push(next)
		} else if self.finishing {
			// if there is no more input, be sure to call `finish` of the
			// tail transform
			self.finishing = false;
			// this is our final output, so we can just return directly (if
			// there is `more` output, `drain` will handle it)
			self.tail.finish()
		} else {
			// no output for now
			Chars::None
		}
	}
}
