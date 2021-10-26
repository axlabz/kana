//! Build test for [`table!`].

use kana_macros::table;

// Basic table
table! {
	basic => {
		'a': 'A',
		"b": "B",
	}
}

fn main() {
	println!("Build test for the `table!` macro");
}
