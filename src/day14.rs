use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn parse_mask(line: String) -> (u64, u64) {
	let (mut mask0, mut mask1) = (u64::MAX, 0);
	for (i, c) in line[7..].chars().enumerate() {
		match c {
			'0' => mask0 -= 1 << (35-i),
			'1' => mask1 += 1 << (35-i),
			_ => {},
		}
	}
	return (mask0, mask1);
}

#[wasm_bindgen(js_name = day14_part_one)]
pub fn part_one(input: String) -> String {
	let mut mask = (0, 0);
	let mut mem: HashMap<&str, u64> = HashMap::new();
	for line in input.lines() {
		if line.contains("mask = ") {
			mask = parse_mask(line.to_string());
		} else {
			let parts: Vec<&str> = line.split("] = ").collect();
			let value: u64 = parts[1].parse().unwrap();
			mem.insert(parts[0], (value&mask.0)|mask.1);
		}
	}
	let mut result = 0;
	for (_, value) in mem.iter() {
		result += value;
	}
	return result.to_string();
}

fn apply_mask(addr: &String, mask: &String) -> Vec<String> {
	let mut result = vec![String::new()]; 
	for (c, m) in addr.chars().zip(mask.chars()) {
		let size = result.len();
		for i in 0..size {
			match m {
				'1' => result[i].push('1'),
				'0' => result[i].push(c),
				'X' => {
					result.push(format!("{}0", result[i]));
					result[i].push('1');
				},
				x => panic!(format!("Invalid character {}", x)),
			}
		}
	}
	return result;
}

#[wasm_bindgen(js_name = day14_part_two)]
pub fn part_two(input: String) -> String {
	let mut mask = String::new();
	let mut mem = HashMap::new();
	for line in input.lines() {
		if line.contains("mask = ") {
			mask = line[7..].to_string();
		} else {
			let parts: Vec<u64> = line[4..].split("] = ").map(|s| s.parse().unwrap()).collect();
			for addr in apply_mask(&format!("{:036b}", parts[0]), &mask) {
				mem.insert(addr, parts[1]);
			}
		}
	}
	let mut result = 0;
	for (_, value) in mem.iter() {
		result += value;
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(14);
	assert_eq!(part_one(input.clone()), "14839536808842".to_string());
	assert_eq!(part_two(input.clone()), "4215284199669".to_string());
}
