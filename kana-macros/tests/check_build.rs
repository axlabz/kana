#[test]
fn test_build() {
	let t = trybuild::TestCases::new();
	t.pass("tests/build_charinfo_syntax.rs");
}
