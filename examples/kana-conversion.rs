use std::{fs::File, io::Read, time::Instant};

use kana::{
	transform::{self, Chainable},
	Text, Transform,
};

fn convert(filename: &'static str) {
	println!("\n>> Testing {}...", filename);

	// read the input file to memory:
	let filename = format!("./testdata/perf/{}", filename);
	let start = Instant::now();
	let mut file = File::open(filename).unwrap();
	let mut text = String::new();
	file.read_to_string(&mut text).unwrap();

	let elapsed = start.elapsed();
	println!("=> file read ({} bytes)\n   took {:?}", text.len(), elapsed);

	println!(
		"L1: {}",
		plain_conversion_lc("あいうえおじじゃじゅじょじじABC")
	);
	println!(
		"L2: {}",
		using_convert_lc("あいうえおじじゃじゅじょじじABC")
	);

	//----[ Lower case ]------------------------------------------------------//

	let start = Instant::now();
	let output = plain_lc(&text);
	let elapsed = start.elapsed();
	println!(
		"=> plain lowercase ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	let start = Instant::now();
	let output = flat_lc(&text);
	let elapsed = start.elapsed();
	println!(
		"=> flat lowercase ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	let start = Instant::now();
	let output = conversion_lc(&text);
	let elapsed = start.elapsed();
	println!(
		"=> conversion lowercase ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	//----[ Plain conversion ]------------------------------------------------//

	let start = Instant::now();
	let output = plain_conversion(&text);
	let elapsed = start.elapsed();
	println!(
		"=> plain conversion ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	//----[ Using convert ]---------------------------------------------------//

	let expected = output;

	let start = Instant::now();
	let output = using_convert(&text);
	let elapsed = start.elapsed();
	println!(
		"=> kana::convert ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	if output != expected {
		panic!("output does not match expected");
	}

	//----[ Plain conversion + lowercase ]------------------------------------//

	let start = Instant::now();
	let output = plain_conversion_lc(&text);
	let elapsed = start.elapsed();
	println!(
		"=> plain conversion lowercased ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	//----[ Using convert + lowercase ]---------------------------------------//

	let expected = output;

	let start = Instant::now();
	let output = using_convert_lc(&text);
	let elapsed = start.elapsed();
	println!(
		"=> kana::convert lowercased ({} bytes)\n   took {:?}",
		output.len(),
		elapsed
	);

	if output != expected {
		panic!("output does not match expected");
	}
}

fn plain_lc(input: &str) -> String {
	input.to_lowercase()
}

fn flat_lc(input: &str) -> String {
	input.chars().flat_map(|c| c.to_lowercase()).collect()
}

fn conversion_lc(input: &str) -> String {
	kana::convert(transform::lower(), input.chars()).collect()
}

fn plain_conversion(mut input: &str) -> String {
	let mut output = String::new();
	loop {
		let mut again = false;
		for (pos, chr) in input.char_indices() {
			match chr {
				'あ' => output.push('a'),
				'い' => output.push('i'),
				'う' => output.push('u'),
				'え' => output.push('e'),
				'お' => output.push('o'),
				'か' => output.push_str("ka"),
				'き' => output.push_str("ki"),
				'く' => output.push_str("ku"),
				'け' => output.push_str("ke"),
				'こ' => output.push_str("ko"),
				_ => {
					let next = &input[pos..];
					if let Some(next) = next.strip_prefix("じゃ") {
						output.push_str("ja");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じょ") {
						output.push_str("jo");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じゅ") {
						output.push_str("ju");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じ") {
						output.push_str("ji");
						input = next;
						again = true;
						break;
					} else {
						output.push(chr);
					}
				}
			}
		}
		if !again {
			break;
		}
	}
	output
}

fn plain_conversion_lc(mut input: &str) -> String {
	let mut output = String::new();
	loop {
		let mut again = false;
		for (pos, chr) in input.char_indices() {
			match chr {
				'あ' => output.push('a'),
				'い' => output.push('i'),
				'う' => output.push('u'),
				'え' => output.push('e'),
				'お' => output.push('o'),
				'か' => output.push_str("ka"),
				'き' => output.push_str("ki"),
				'く' => output.push_str("ku"),
				'け' => output.push_str("ke"),
				'こ' => output.push_str("ko"),
				_ => {
					let next = &input[pos..];
					if let Some(next) = next.strip_prefix("じゃ") {
						output.push_str("ja");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じょ") {
						output.push_str("jo");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じゅ") {
						output.push_str("ju");
						input = next;
						again = true;
						break;
					} else if let Some(next) = next.strip_prefix("じ") {
						output.push_str("ji");
						input = next;
						again = true;
						break;
					} else {
						for chr in chr.to_lowercase() {
							output.push(chr);
						}
					}
				}
			}
		}
		if !again {
			break;
		}
	}
	output
}

fn using_convert(input: &str) -> String {
	kana::convert(KanaConverter {}, input.chars()).collect()
}

fn using_convert_lc(input: &str) -> String {
	kana::convert(KanaConverter {}.chain(transform::lower()), input.chars()).collect()
}

struct KanaConverter {}

struct KanaIter<I: Iterator<Item = Text>> {
	inner: I,
	next: Option<char>,
	is_ji: bool,
}

impl<I: Iterator<Item = Text>> Transform<I> for KanaConverter {
	type Output = KanaIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		KanaIter {
			inner: input,
			next: None,
			is_ji: false,
		}
	}
}

impl<I: Iterator<Item = Text>> KanaIter<I> {
	fn push_char(&mut self, next: char) -> Option<Text> {
		if self.is_ji {
			self.is_ji = false;
			let output = match next {
				'ゃ' => Text::Static("ja"),
				'ょ' => Text::Static("jo"),
				'ゅ' => Text::Static("ju"),
				_ => {
					self.next = Some(next);
					Text::Static("ji")
				}
			};
			return Some(output);
		} else {
			let output = match next {
				'あ' => Text::Char('a'),
				'い' => Text::Char('i'),
				'う' => Text::Char('u'),
				'え' => Text::Char('e'),
				'お' => Text::Char('o'),
				'か' => Text::Static("ka"),
				'き' => Text::Static("ki"),
				'く' => Text::Static("ku"),
				'け' => Text::Static("ke"),
				'こ' => Text::Static("ko"),
				'じ' => {
					self.is_ji = true;
					return self.next();
				}
				_ => Text::Char(next),
			};
			return Some(output);
		}
	}
}

impl<I: Iterator<Item = Text>> Iterator for KanaIter<I> {
	type Item = Text;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(next) = std::mem::take(&mut self.next) {
			return self.push_char(next);
		}

		match self.inner.next() {
			None => {
				return None;
			}
			Some(Text::Static(_)) => {
				unimplemented!()
			}
			Some(Text::Char(next)) => {
				return self.push_char(next);
			}
		}
	}
}

fn main() {
	convert("kanjidic.dat");
}
