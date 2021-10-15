use std::collections::HashSet;
use std::iter::once;

/// Helper to build the set of ranges for the [`crate::charinfo!`] macro.
pub struct RangeBuilder {
	/// Sorted and non-overlapping list of ranges added to the builder.
	ranges: Vec<Range>,
}

/// Internal range for [`RangeBuilder`].
struct Range {
	/// Start character of the range.
	start: char,
	/// End character of the range, exclusive.
	end: char,
	/// Set of flags applied to this range.
	flags: HashSet<String>,
}

impl RangeBuilder {
	pub fn new() -> Self {
		RangeBuilder { ranges: Vec::new() }
	}

	/// Returns an iterator for the list of ranges.
	pub fn ranges(&self) -> impl Iterator<Item = (char, char, Vec<&str>)> {
		self.ranges.iter().map(|x| {
			let mut flags = x.flags.iter().map(|x| x.as_str()).collect::<Vec<_>>();
			flags.sort();
			(x.start, x.end, flags)
		})
	}

	/// Add a specific flag for the given character range. Note that the range
	/// is exclusive.
	pub fn add(mut self, mut start: char, end: char, flag: &str) -> Self {
		self.ranges = self
			.ranges
			.into_iter()
			// Ranges are sorted. We use the `flat_map` to map the new range
			// segments and/or split existing ranges.
			.flat_map(|mut cur| {
				let mut out = Vec::new();

				// did we finish mapping the new range?
				let has_range = start < end;

				// does the new range have a portion before the current one?
				if has_range && start < cur.start {
					// add a new range to map it
					out.push(Range {
						start,
						end: std::cmp::min(end, cur.start),
						flags: once(flag.to_string()).collect(),
					});
				}

				let next_start = cur.end;

				// does the new range intersect the current one? also check if
				// the flag is actually a new one
				if has_range && start < cur.end && end > cur.start && !cur.flags.contains(flag) {
					// is current range entirely contained within the new one?
					let is_contained = cur.start >= start && cur.end <= end;
					if is_contained {
						// if the new range covers the current one then we just
						// need to add the new flag
						cur.flags.insert(flag.to_string());
						out.push(cur);
					} else {
						// merged flags
						let mut merged = cur.flags.clone();
						merged.insert(flag.to_string());

						if start > cur.start && end < cur.end {
							// the new range is inside the current range, we need
							// to split into three ranges
							out.push(Range {
								start: cur.start,
								end: start,
								flags: cur.flags.clone(),
							});
							out.push(Range {
								start: start,
								end: end,
								flags: merged,
							});
							out.push(Range {
								start: end,
								end: cur.end,
								flags: cur.flags,
							})
						} else if start > cur.start {
							// split in two, overlap at the end
							out.push(Range {
								start: cur.start,
								end: start,
								flags: cur.flags,
							});
							out.push(Range {
								start: start,
								end: cur.end,
								flags: merged,
							});
						} else {
							// split in two, overlap at the start
							out.push(Range {
								start: cur.start,
								end: end,
								flags: merged,
							});
							out.push(Range {
								start: end,
								end: cur.end,
								flags: cur.flags,
							});
						}
					}
				} else {
					out.push(cur);
				}

				// next iteration starts at the end of the current range
				if start < next_start {
					start = next_start;
				}

				out
			})
			.collect();

		// if there is any part of the new range remaining, just add it
		if start < end {
			self.ranges.push(Range {
				start,
				end,
				flags: once(flag.to_string()).collect(),
			})
		}
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn builder_empty() {
		let x = RangeBuilder::new();
		assert_eq!(x.ranges().count(), 0);
	}

	#[test]
	fn builder_non_overlapping() {
		// single range
		let x = RangeBuilder::new().add('a', 'c', "f1");
		assert_eq!(x.ranges().collect::<Vec<_>>(), vec![('a', 'c', vec!["f1"])]);

		// add flag to same range
		let x = x.add('a', 'c', "f2");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![('a', 'c', vec!["f1", "f2"])]
		);

		// add non-overlapping range after
		let x = x.add('x', 'z', "xy");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![('a', 'c', vec!["f1", "f2"]), ('x', 'z', vec!["xy"])]
		);

		// add non-overlapping range before
		let x = x.add('0', '9', "num");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('0', '9', vec!["num"]),
				('a', 'c', vec!["f1", "f2"]),
				('x', 'z', vec!["xy"])
			]
		);

		// add non-overlapping range in the middle
		let x = x.add('i', 'j', "x");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('0', '9', vec!["num"]),
				('a', 'c', vec!["f1", "f2"]),
				('i', 'j', vec!["x"]),
				('x', 'z', vec!["xy"])
			]
		);
	}

	#[test]
	fn builder_sequential() {
		let x = RangeBuilder::new()
			.add('1', '2', "1")
			.add('2', '3', "2")
			.add('3', '4', "3");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('1', '2', vec!["1"]),
				('2', '3', vec!["2"]),
				('3', '4', vec!["3"]),
			]
		);

		let x = x.add('7', '8', "7").add('6', '7', "6").add('8', '9', "8");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('1', '2', vec!["1"]),
				('2', '3', vec!["2"]),
				('3', '4', vec!["3"]),
				('6', '7', vec!["6"]),
				('7', '8', vec!["7"]),
				('8', '9', vec!["8"]),
			]
		);

		let x = x.add('5', '6', "5").add('4', '5', "4");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('1', '2', vec!["1"]),
				('2', '3', vec!["2"]),
				('3', '4', vec!["3"]),
				('4', '5', vec!["4"]),
				('5', '6', vec!["5"]),
				('6', '7', vec!["6"]),
				('7', '8', vec!["7"]),
				('8', '9', vec!["8"]),
			]
		);
	}

	#[test]
	fn builder_overlapping() {
		// add overlapping at the start and end
		let x = RangeBuilder::new()
			.add('A', 'Z', "az")
			.add('A', 'C', "ac")
			.add('X', 'Z', "xz");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('A', 'C', vec!["ac", "az"]),
				('C', 'X', vec!["az"]),
				('X', 'Z', vec!["az", "xz"]),
			]
		);

		// add overlapping in the middle and the whole range
		let x = x.add('M', 'P', "mp").add('A', 'Z', "xx");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('A', 'C', vec!["ac", "az", "xx"]),
				('C', 'M', vec!["az", "xx"]),
				('M', 'P', vec!["az", "mp", "xx"]),
				('P', 'X', vec!["az", "xx"]),
				('X', 'Z', vec!["az", "xx", "xz"]),
			]
		);

		// create some non overlapping ranges
		let x = x.add('3', '6', "36").add('q', 't', "qt");
		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('3', '6', vec!["36"]),
				('A', 'C', vec!["ac", "az", "xx"]),
				('C', 'M', vec!["az", "xx"]),
				('M', 'P', vec!["az", "mp", "xx"]),
				('P', 'X', vec!["az", "xx"]),
				('X', 'Z', vec!["az", "xx", "xz"]),
				('q', 't', vec!["qt"]),
			]
		);

		// create a range covering all others
		let x = x.add('0', 'z', "00");

		assert_eq!(
			x.ranges().collect::<Vec<_>>(),
			vec![
				('0', '3', vec!["00"]),
				('3', '6', vec!["00", "36"]),
				('6', 'A', vec!["00"]),
				('A', 'C', vec!["00", "ac", "az", "xx"]),
				('C', 'M', vec!["00", "az", "xx"]),
				('M', 'P', vec!["00", "az", "mp", "xx"]),
				('P', 'X', vec!["00", "az", "xx"]),
				('X', 'Z', vec!["00", "az", "xx", "xz"]),
				('Z', 'q', vec!["00"]),
				('q', 't', vec!["00", "qt"]),
				('t', 'z', vec!["00"]),
			]
		);
	}
}
