use regex::Regex;
use wasm_bindgen::prelude::*;

const DEPTH: usize = 5;

fn calc_rules<'a>(i: usize, mut rules_memo: &'a mut Vec<String>, rules: &Vec<&str>) -> &'a mut Vec<String> {
	let recursive = Regex::new("^r+$").unwrap().is_match(&rules_memo[i]);
	if rules_memo[i].len() != 0 && !recursive || recursive && rules_memo[i].len() >= DEPTH {
		return rules_memo;
	}
	rules_memo[i] += "r";
	let sub: Vec<&str> = rules[i].split(":").collect::<Vec<&str>>()[1].split_whitespace().collect();
	let mut result = "(".to_string();
	for part in sub {
		if let Ok(n) = part.parse::<usize>() {
			rules_memo = calc_rules(n, rules_memo, rules);
			result += &rules_memo[n];
		} else {
			result += part.trim_matches('"');
		}
	}

	rules_memo[i] = result + ")";
	return rules_memo;
}

#[wasm_bindgen(js_name = day19_part_one)]
pub fn part_one(input: String) -> String {
	let parts: Vec<&str> = input.split("\n\n").collect();

	let mut rules: Vec<&str> = parts[0].lines().collect();
	rules.sort_by_key(|line| line.split(":").next().unwrap().parse::<usize>().unwrap());

	let mut rules_memo = &mut vec![String::new(); rules.len()];
	rules_memo = calc_rules(0, rules_memo, &rules);

	let re = Regex::new(&format!("^{}$", rules_memo[0])).unwrap();
	let mut result = 0;
	for line in parts[1].lines() {
		if re.is_match(line) {
			result += 1;
		}
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day19_part_two)]
pub fn part_two(input: String) -> String {
	let tmp = regex::Regex::new("\n8:.*\n").unwrap().replace(&input, "\n8: 42 | 42 8\n");
	return part_one(regex::Regex::new("\n11:.*\n").unwrap().replace(&tmp, "\n11: 42 31 | 42 11 31\n").to_string());
}

#[test]
fn test() {
	let input = crate::common::get_input(19);
	assert_eq!(part_one(input.clone()), "171".to_string());
	assert_eq!(part_two(input.clone()), "369".to_string());
}
