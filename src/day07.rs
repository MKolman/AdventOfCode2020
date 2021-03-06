use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> HashMap<String, Vec<(u16, String)>> {
	let mut result = HashMap::new();
	for line in input.lines() {
		let bags: Vec<&str> = line.splitn(2, " bags contain ").collect();
		result.insert(bags[0].to_string(), Vec::new());
		for inside in bags[1].split(',') {
			let words: Vec<&str> = inside.trim().split_whitespace().collect();
			if words[0] == "no" {
				continue;
			}
			result.get_mut(&bags[0].to_string()).unwrap().push((
				words[0].parse().unwrap(),
				words[1].to_owned() + " " + words[2],
			));
		}
	}
	return result;
}

#[wasm_bindgen(js_name = day07_part_one)]
pub fn part_one(input: &str) -> String {
	let graph = parse_input(&input);
	let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
	for (outer, contents) in &graph {
		for (_, inner) in contents {
			let bags = reverse.entry(inner.to_string()).or_insert(Vec::new());
			bags.push(outer.to_string());
		}
	}
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back("shiny gold".to_string());
	while let Some(color) = queue.pop_front() {
		visited.insert(color.clone());
		if !reverse.contains_key(&color) {
			continue;
		}
		for next in &reverse[&color] {
			if !visited.contains(next) {
				queue.push_back(next.clone());
			}
		}
	}
	return (visited.len() - 1).to_string();
}

#[wasm_bindgen(js_name = day07_part_two)]
pub fn part_two(input: &str) -> String {
	let graph = parse_input(&input);
	fn inside(
		color: &str,
		graph: &HashMap<String, Vec<(u16, String)>>,
		memo: &mut HashMap<String, u64>,
	) -> u64 {
		if !memo.contains_key(color) {
			let val = graph[color]
				.iter()
				.map(|(num, clr)| (*num as u64) + (*num as u64) * inside(clr, graph, memo))
				.sum();
			memo.insert(color.to_string(), val);
		}
		return memo[color];
	};
	return inside(&"shiny gold".to_string(), &graph, &mut HashMap::new()).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(7);
	assert_eq!(part_one(&input), "370".to_string());
	assert_eq!(part_two(&input), "29547".to_string());
}
