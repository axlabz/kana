use std::{fs::File, io::Read, time::Instant};

use kana::{
	transform::{self, Chainable},
	Chars, Transform,
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
	kana::convert(KanaConverter::new(), input.chars()).collect()
}

fn using_convert_lc(input: &str) -> String {
	kana::convert(
		KanaConverter::new().chain(transform::lower()),
		input.chars(),
	)
	.collect()
}

struct KanaConverter {
	next: Option<char>,
	is_ji: bool,
}

impl Transform for KanaConverter {
	fn push(&mut self, next: char) -> Chars {
		self.push_char(next)
	}

	fn drain(&mut self) -> Chars {
		if let Some(next) = self.next {
			self.next = None;
			self.push_char(next)
		} else {
			Chars::None
		}
	}
}

impl KanaConverter {
	fn new() -> Self {
		Self {
			next: None,
			is_ji: false,
		}
	}

	fn push_char(&mut self, next: char) -> Chars {
		if self.is_ji {
			self.is_ji = false;
			match next {
				'ゃ' => Chars::L2(['j', 'a'], false),
				'ょ' => Chars::L2(['j', 'o'], false),
				'ゅ' => Chars::L2(['j', 'u'], false),
				_ => {
					self.next = Some(next);
					Chars::L2(['j', 'i'], true)
				}
			}
		} else {
			match next {
				'あ' => Chars::L1(['a'], false),
				'い' => Chars::L1(['i'], false),
				'う' => Chars::L1(['u'], false),
				'え' => Chars::L1(['e'], false),
				'お' => Chars::L1(['o'], false),
				'か' => Chars::L2(['k', 'a'], false),
				'き' => Chars::L2(['k', 'i'], false),
				'く' => Chars::L2(['k', 'u'], false),
				'け' => Chars::L2(['k', 'e'], false),
				'こ' => Chars::L2(['k', 'o'], false),
				'じ' => {
					self.is_ji = true;
					Chars::None
				}
				_ => Chars::L1([next], false),
			}
		}
	}
}

fn main() {
	convert("kanjidic.dat");
}
