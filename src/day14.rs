use std::collections::HashMap;
use wasm_bindgen::prelude::*;

enum Cmd<'a> {
	Mask(&'a str),
	Assign(u64, u64),
}

fn parse_input(input: &str) -> Vec<Cmd> {
	let mut result = Vec::new();
	for line in input.lines() {
		if line.contains("mask = ") {
			result.push(Cmd::Mask(&line[7..]));
		} else {
			let parts: Vec<&str> = line.split("] = ").collect();
			result.push(Cmd::Assign(
				parts[0][4..].parse().unwrap(),
				parts[1].parse().unwrap(),
			));
		}
	}
	return result;
}

fn parse_mask(line: &str) -> (u64, u64) {
	let (mut mask0, mut mask1) = (u64::MAX, 0);
	for (i, c) in line.chars().enumerate() {
		match c {
			'0' => mask0 -= 1 << (35 - i),
			'1' => mask1 += 1 << (35 - i),
			_ => {}
		}
	}
	return (mask0, mask1);
}

#[wasm_bindgen(js_name = day14_part_one)]
pub fn part_one(input: &str) -> String {
	let mut mask = (0, 0);
	let mut mem: HashMap<u64, u64> = HashMap::new();
	for cmd in parse_input(input) {
		match cmd {
			Cmd::Mask(m) => mask = parse_mask(m),
			Cmd::Assign(addr, val) => {
				mem.insert(addr, (val & mask.0) | mask.1);
			}
		};
	}
	let mut result = 0;
	for (_, value) in mem.iter() {
		result += value;
	}
	return result.to_string();
}

fn apply_mask(addr: &str, mask: &str) -> Vec<String> {
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
				}
				x => panic!(format!("Invalid character {}", x)),
			}
		}
	}
	return result;
}

#[wasm_bindgen(js_name = day14_part_two)]
pub fn part_two(input: &str) -> String {
	let mut mask = "";
	let mut mem = HashMap::new();
	for cmd in parse_input(input) {
		match cmd {
			Cmd::Mask(m) => mask = m,
			Cmd::Assign(addr, val) => {
				for addr in apply_mask(&format!("{:036b}", addr), mask) {
					mem.insert(addr, val);
				}
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
	assert_eq!(part_one(&input), "14839536808842".to_string());
	assert_eq!(part_two(&input), "4215284199669".to_string());
}
