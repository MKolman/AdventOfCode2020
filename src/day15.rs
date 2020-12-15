use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn solve(input: &String, length: usize) -> String {
	let mut last: HashMap<u64, usize> = HashMap::new();
	let nums: Vec<u64> = input.split(",").map(|n| n.parse().unwrap()).collect();
	for (i, n) in nums[..nums.len()-1].iter().enumerate() {
		last.insert(*n, i);
	}
	let mut prev = nums[nums.len()-1];
	for i in nums.len()-1..length-1 {
		println!("{:?}", prev);
		if last.contains_key(&prev) {
			let next = i - *last.get(&prev).unwrap();
			last.insert(prev, i);
			prev = next as u64;
		} else {
			last.insert(prev, i);
			prev = 0;
		}
	}
	return prev.to_string();
}

#[wasm_bindgen(js_name = day15_part_one)]
pub fn part_one(input: String) -> String {
	return solve(&input, 2020);
}

#[wasm_bindgen(js_name = day15_part_two)]
pub fn part_two(input: String) -> String {
	return solve(&input, 30000000);
}

#[test]
fn test() {
	let input = crate::common::get_input(15);
	assert_eq!(part_one(input.clone()), "492".to_string());
	assert_eq!(part_two(input.clone()), "63644".to_string());
}
