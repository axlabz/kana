use automato_macros::transducer;

fn main() {
	transducer!(sample:
		"a" => "A",
		"b" => "B",
		"ab" => "C",
	);

	// spell-checker: ignore
	let out = sample::new("aabbabab123".chars()).collect::<String>();
	println!("{}", out);
}
