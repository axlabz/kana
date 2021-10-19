use std::char::{ToLowercase, ToUppercase};

use crate::{Chars, Transform};

/// Lower-case the input text.
pub fn lower() -> impl Transform {
	Lower { iter: None }
}

/// Upper-case the input text.
pub fn upper() -> impl Transform {
	Upper { iter: None }
}

struct Lower {
	iter: Option<ToLowercase>,
}

impl Transform for Lower {
	fn push(&mut self, next: char) -> Chars {
		self.iter = Some(next.to_lowercase());
		self.drain()
	}

	fn drain(&mut self) -> Chars {
		if let Some(iter) = &mut self.iter {
			Chars::read_iter(iter)
		} else {
			Chars::None
		}
	}
}

struct Upper {
	iter: Option<ToUppercase>,
}

impl Transform for Upper {
	fn push(&mut self, next: char) -> Chars {
		self.iter = Some(next.to_uppercase());
		self.drain()
	}

	fn drain(&mut self) -> Chars {
		if let Some(iter) = &mut self.iter {
			Chars::read_iter(iter)
		} else {
			Chars::None
		}
	}
}
