use regex::Regex;
use wasm_bindgen::prelude::*;

fn calc_rules<'a>(i: usize, mut rules_memo: &'a mut Vec<String>, rules: &Vec<&str>) -> &'a mut Vec<String> {
	if rules_memo[i].len() != 0 {
		return rules_memo;
	}
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
pub fn part_two(mut input: String) -> String {
	// 8: 42 | 42 8  => 8: 42 +
	input = input.replace("\n8: 42", "\n8: 42 +");
	// 11: 42 31 | 42 11 31  => 11: 42 31 | 42 {2} 31 {2} | 42 {3} 31 {3} | (and so on)
	let mut rule11 = "\n11: 42 31".to_string();
	for n in 2..=5 {
		rule11 += &format!(" | 42 {{{}}} 31 {{{}}}", n, n);
	}
	input = input.replace("\n11: 42 31", &rule11);
	return part_one(input);
}

#[test]
fn test() {
	let input = crate::common::get_input(19);
	assert_eq!(part_one(input.clone()), "171".to_string());
	assert_eq!(part_two(input.clone()), "369".to_string());
}
