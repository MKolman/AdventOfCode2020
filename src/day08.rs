use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day08_part_one)]
pub fn part_one(input: String) -> String {
	let code: Vec<(&str, i64)> = input.lines().map(|l| {
		let pieces: Vec<&str> = l.splitn(2, " ").collect();
		(pieces[0], pieces[1].parse().unwrap())
	}).collect();
	let mut pointer = 0_i64;
	let mut acc = 0;
	let mut visited = HashSet::new();
	while !visited.contains(&pointer) {
		visited.insert(pointer);
		match code[pointer as usize] {
			("acc", n) => {
				acc += n;
				pointer += 1;
			},
			("jmp", n) => pointer += n,
			_ => pointer += 1
		}
	}
	return acc.to_string()
}

#[wasm_bindgen(js_name = day08_part_two)]
pub fn part_two(input: String) -> String {
	let mut code: Vec<(&str, i64)> = input.lines().map(|l| {
		let pieces: Vec<&str> = l.splitn(2, " ").collect();
		(pieces[0], pieces[1].parse().unwrap())
	}).collect();
	for i in 0..code.len() {
		code[i].0 = match code[i].0 {
			"jmp" => "nop",
			"nop" => "jmp",
			c => c
		};
		let mut pointer = 0_i64;
		let mut acc = 0;
		let mut visited = HashSet::new();
		while !visited.contains(&pointer) {
			if pointer >= code.len() as i64 { return acc.to_string(); }
			visited.insert(pointer);
			match code[pointer as usize] {
				("acc", n) => {
					acc += n;
					pointer += 1;
				},
				("jmp", n) => pointer += n,
				_ => pointer += 1
			}
		}
		code[i].0 = match code[i].0 {
			"jmp" => "nop",
			"nop" => "jmp",
			c => c
		};
	}
	return "0".to_string()
}

#[test]
fn test() {
	let input = crate::common::get_input(8);
	assert_eq!(part_one(input.clone()), "1331".to_string());
	assert_eq!(part_two(input.clone()), "1121".to_string());
}
