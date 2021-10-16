#[test]
fn can_build_syntax() {
	let t = trybuild::TestCases::new();
	t.pass("tests/build_charinfo_syntax.rs");
}
