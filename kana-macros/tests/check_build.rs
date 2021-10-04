#[test]
fn test_build() {
	let t = trybuild::TestCases::new();
	t.pass("tests/01-build_charinfo_simple.rs");
}
