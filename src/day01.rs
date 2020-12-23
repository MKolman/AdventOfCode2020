use std::collections::HashSet;
use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
	input.lines().map(|v| v.parse().unwrap()).collect()
}

#[wasm_bindgen(js_name = day01_part_one)]
pub fn part_one(input: &str) -> String {
	let mut set = HashSet::new();
	let vec = parse_input(&input);
	for n in vec {
		if set.contains(&(2020 - n)) {
			return (n * (2020 - n)).to_string();
		}
		set.insert(n);
	}
	return "0".to_string();
}

#[wasm_bindgen(js_name = day01_part_two)]
pub fn part_two(input: &str) -> String {
	let mut set = HashSet::new();
	let vec = parse_input(&input);
	for (i, n) in vec.iter().enumerate() {
		for m in &vec[i + 1..vec.len()] {
			if set.contains(&(2020 - n - m)) {
				return (n * m * (2020 - n - m)).to_string();
			}
		}
		set.insert(n);
	}
	return "0".to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(1);
	assert_eq!(part_one(&input), "787776");
	assert_eq!(part_two(&input), "262738554");
}
