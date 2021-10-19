use kana::{
	convert,
	transform::{self, Chainable},
};

#[test]
fn simple_transform() {
	let input = "ABC DEF".chars();
	let rules = transform::lower();
	let result = convert(rules, input).collect::<String>();
	assert_eq!("abc def", result);
}

#[test]
fn transform_lower() {
	let input = "ABC DEF".chars();
	let rules = transform::lower();
	let result = convert(rules, input).collect::<String>();
	assert_eq!("abc def", result);
}

#[test]
fn transform_upper() {
	let input = "abc def".chars();
	let rules = transform::upper();
	let result = convert(rules, input).collect::<String>();
	assert_eq!("ABC DEF", result);
}

#[test]
fn transform_trim() {
	let input = "  abc   def 123   !   ".chars();
	let rules = transform::trim();
	let result = convert(rules, input).collect::<String>();
	assert_eq!("abc def 123 !", result);
}

#[test]
fn simple_chaining() {
	let input = "  ABC  DEF\t \t123  ".chars();
	let rules = transform::lower().chain(transform::trim());
	let result = convert(rules, input).collect::<String>();
	assert_eq!("abc def 123", result);
}
