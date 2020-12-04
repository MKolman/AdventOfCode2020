use std::collections::HashMap;
use regex::Regex;
use wasm_bindgen::prelude::*;

fn parse_input(input: &String) -> Vec<HashMap<String, String>> {
	let mut result = vec![HashMap::new()];
	let mut size = 0;
	for line in input.lines() {
		if line.trim() == "" {
			size += 1;
			result.push(HashMap::new());
		}
		for kv in line.split_whitespace() {
			let pair: Vec<&str> = kv.split(':').collect();
			result[size].insert(pair[0].to_string(), pair[1].to_string());
		}
	}
	return result;
}

#[wasm_bindgen(js_name = day04_part_one)]
pub fn part_one(input: String) -> String {
	let data = parse_input(&input);
	let mut result = 0;
	for pass in &data {
		if pass.len() as i64 - (pass.contains_key("cid") as i64) == 7 {
			result += 1;
		}
	}
	return result.to_string()
}


#[wasm_bindgen(js_name = day04_part_two)]
pub fn part_two(input: String) -> String {
	let rules = [
		("byr", Regex::new(r"^(19[2-9][0-9]|200[012])$").unwrap()),
		("iyr", Regex::new(r"^20(1[0-9]|20)$").unwrap()),
		("eyr", Regex::new(r"^20(2[0-9]|30)$").unwrap()),
		("hgt", Regex::new(r"^(1([5678][0-9]|9[0123])cm|(59|6[0-9]|7[0-6])in)$").unwrap()),
		("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
		("ecl", Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
		("pid", Regex::new(r"^[0-9]{9}$").unwrap()),
	];
	let data = parse_input(&input);
	let mut result = 0;
	for pass in &data {
		if pass.len() as i64 - (pass.contains_key("cid") as i64) == 7 {
			result += 1;
			for (key, rule) in &rules {
				// println!("{}, {:?}", key, pass);
				if !rule.is_match(pass.get(&key.to_string()).unwrap()) {
					result -= 1;
					break;
				}
			}
		}
	}
	return result.to_string()
}

#[test]
fn test() {
	let input = crate::common::get_input(4);
	assert_eq!(part_one(input.clone()), "228");
	assert_eq!(part_two(input.clone()), "175");
}
