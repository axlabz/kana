use kana_macros::answer;

#[test]
fn test_answer() {
	assert_eq!(answer!(), 42);
}
