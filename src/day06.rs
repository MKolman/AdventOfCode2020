use std::collections::HashSet;
use std::ops::BitAnd;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day06_part_one)]
pub fn part_one(input: String) -> String {
	let groups: Vec<HashSet<char>> = input
		.split("\n\n")
		.map(|group| {
			let mut set: HashSet<char> = group.to_string().chars().collect();
			set.remove(&'\n');
			return set;
		})
		.collect();
	groups
		.iter()
		.fold(0, |sum, group| sum + group.len())
		.to_string()
}

#[wasm_bindgen(js_name = day06_part_two)]
pub fn part_two(input: String) -> String {
	let groups: Vec<HashSet<char>> = input
		.split("\n\n")
		.map(|group| {
			group
				.lines()
				.fold(group.chars().collect::<HashSet<char>>(), |all, line| {
					all.bitand(&line.chars().collect())
				})
		})
		.collect();
	groups
		.iter()
		.fold(0, |sum, group| sum + group.len())
		.to_string()
}

#[test]
fn test() {
	let input = crate::common::get_input(6);
	assert_eq!(part_one(input.clone()), "6782");
	assert_eq!(part_two(input.clone()), "3596");
}
