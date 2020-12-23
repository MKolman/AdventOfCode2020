use wasm_bindgen::prelude::*;

fn solve(input: &str, length: usize) -> String {
	let unseen = length + 1;
	let mut seen = vec![unseen; length];
	let start_len = input.matches(',').count();
	let mut prev = 0;
	for (i, n) in input.split(',').enumerate() {
		let p = n.parse().unwrap();
		if i == start_len {
			prev = p;
		} else {
			seen[p] = i;
		}
	}
	for i in start_len..length - 1 {
		if seen[prev] != unseen {
			let next = i - seen[prev];
			seen[prev] = i;
			prev = next;
		} else {
			seen[prev] = i;
			prev = 0;
		}
	}
	return prev.to_string();
}

#[wasm_bindgen(js_name = day15_part_one)]
pub fn part_one(input: &str) -> String {
	return solve(&input, 2020);
}

#[wasm_bindgen(js_name = day15_part_two)]
pub fn part_two(input: &str) -> String {
	return solve(&input, 30000000);
}

#[test]
fn test() {
	let input = crate::common::get_input(15);
	assert_eq!(part_one(&input), "492".to_string());
	assert_eq!(part_two(&input), "63644".to_string());
}
