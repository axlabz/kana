use std::{fs::File, io::Read, time::Instant};

use kana::{
	transform::{self, Chainable},
	Transform,
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

struct KanaIter<I: Iterator<Item = char>> {
	inner: I,
	ahead: Option<char>,
	input: Option<char>,
	is_ji: bool,
}

impl<I: Iterator<Item = char>> Transform<I> for KanaConverter {
	type Output = KanaIter<I>;

	fn convert(&self, input: I) -> Self::Output {
		KanaIter {
			inner: input,
			ahead: None,
			input: None,
			is_ji: false,
		}
	}
}

impl<I: Iterator<Item = char>> Iterator for KanaIter<I> {
	type Item = char;

	fn next(&mut self) -> Option<char> {
		if let Some(char) = self.ahead {
			self.ahead = None;
			return Some(char);
		}

		let next = if let Some(char) = self.input {
			self.input = None;
			Some(char)
		} else {
			self.inner.next()
		};
		match next {
			None => None,
			Some(next) => {
				if self.is_ji {
					self.is_ji = false;
					match next {
						'ゃ' => {
							self.ahead = Some('a');
							return Some('j');
						}
						'ょ' => {
							self.ahead = Some('o');
							return Some('j');
						}
						'ゅ' => {
							self.ahead = Some('u');
							return Some('j');
						}
						_ => {
							self.input = Some(next);
							self.ahead = Some('i');
							return Some('j');
						}
					};
				} else {
					match next {
						'あ' => return Some('a'),
						'い' => return Some('i'),
						'う' => return Some('u'),
						'え' => return Some('e'),
						'お' => return Some('o'),
						'か' => {
							self.ahead = Some('a');
							return Some('k');
						}
						'き' => {
							self.ahead = Some('i');
							return Some('k');
						}
						'く' => {
							self.ahead = Some('u');
							return Some('k');
						}
						'け' => {
							self.ahead = Some('e');
							return Some('k');
						}
						'こ' => {
							self.ahead = Some('o');
							return Some('k');
						}
						'じ' => {
							self.is_ji = true;
							self.next()
						}
						_ => Some(next),
					}
				}
			}
		}
	}
}

fn main() {
	convert("kanjidic.dat");
}
