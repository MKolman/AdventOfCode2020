use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn parse_input(input: &String) -> HashMap<String, Vec<(u16, String)>> {
	let mut result = HashMap::new();
	for line in input.lines() {
		let bags: Vec<&str> = line.splitn(2, " bags contain ").collect();
		result.insert(bags[0].to_string(), Vec::new());
		for inside in bags[1].split(',') {
			let words: Vec<&str> = inside.trim().split_whitespace().collect();
			if words[0] == "no" { continue; }
			result.get_mut(&bags[0].to_string()).unwrap().push((words[0].parse().unwrap(), words[1].to_owned()+" "+words[2]));
		}
	}
	return result;
}

#[wasm_bindgen(js_name = day07_part_one)]
pub fn part_one(input: String) -> String {
	let graph = parse_input(&input);
	let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
	for (outer, contents) in &graph {
		for (_, inner) in contents {
			if !reverse.contains_key(&inner.to_string()) { reverse.insert(inner.to_string(), Vec::new()); };
			reverse.get_mut(inner).unwrap().push(outer.to_string());
		}
	}
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back("shiny gold".to_string());
	while queue.len() > 0 {
		let color = queue.pop_front().unwrap().to_string();
		visited.insert(color.clone());
		if !reverse.contains_key(&color) { continue; }
		for next in &reverse[&color] {
			if !visited.contains(&next.to_string()) {
				queue.push_back(next.to_string());
			}
		}
	}
	println!("{:?}", visited);
	return (visited.len()-1).to_string();
}

#[wasm_bindgen(js_name = day07_part_two)]
pub fn part_two(input: String) -> String {
	let graph = parse_input(&input);
	let mut count = 0;
	let mut queue = VecDeque::new();
	queue.push_back((1, "shiny gold".to_string()));
	while queue.len() > 0 {
		let (mul, color) = queue.pop_front().unwrap();
		for (num, clr) in &graph[&color.to_string()] {
			count += mul*num;
			queue.push_back((mul*num, clr.to_string()));
		}
	}
	return count.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(7);
	assert_eq!(part_one(input.clone()), "370".to_string());
	assert_eq!(part_two(input.clone()), "29547".to_string());
}
