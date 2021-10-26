#[test]
fn can_build_charinfo_syntax() {
	let t = trybuild::TestCases::new();
	t.pass("tests/build_charinfo_syntax.rs");
}

#[test]
fn can_build_table_syntax() {
	let t = trybuild::TestCases::new();
	t.pass("tests/build_table_syntax.rs");
}
