use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day21_part_one)]
pub fn part_one(input: &str) -> String {
	let mut alergens: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut counter: HashMap<&str, usize> = HashMap::new();
	for line in input.lines() {
		let parts: Vec<_> = line.split(" (contains ").collect();
		let ing: HashSet<_> = parts[0].split(' ').collect();
		for i in &ing {
			if let Some(c) = counter.get_mut(i) {
				*c += 1;
			} else {
				counter.insert(i, 1);
			}
		}
		for alergen in parts[1][..parts[1].len() - 1].split(", ") {
			if let Some(x) = alergens.get_mut(alergen) {
				for i in x.clone().iter() {
					if !ing.contains(i) {
						x.remove(i);
					}
				}
			} else {
				alergens.insert(alergen, ing.clone());
			}
		}
	}
	for ing in alergens.values() {
		for i in ing {
			*counter.get_mut(i).unwrap() = 0;
		}
	}
	return counter.values().sum::<usize>().to_string();
}

#[wasm_bindgen(js_name = day21_part_two)]
pub fn part_two(input: &str) -> String {
	let mut alergens: HashMap<&str, HashSet<&str>> = HashMap::new();
	for line in input.lines() {
		let parts: Vec<_> = line.split(" (contains ").collect();
		let ing: HashSet<_> = parts[0].split(' ').collect();
		for alergen in parts[1][..parts[1].len() - 1].split(", ") {
			if let Some(x) = alergens.get_mut(alergen) {
				for i in x.clone().iter() {
					if !ing.contains(i) {
						x.remove(i);
					}
				}
			} else {
				alergens.insert(alergen, ing.clone());
			}
		}
	}
	let mut alergen_result = HashMap::new();
	for a in alergens.keys() {
		alergen_result.insert(*a, "");
	}
	while !alergens.is_empty() {
		for a in alergens.clone().keys() {
			if alergens[a].len() == 1 {
				let i = *alergens[a].iter().next().unwrap();
				*alergen_result.get_mut(a).unwrap() = i;
				alergens.remove(a);
				for v in alergens.values_mut() {
					v.remove(i);
				}
			}
		}
	}
	let mut result: Vec<_> = alergen_result.iter().collect();
	result.sort();
	return result.iter().map(|(_, &s)| s).collect::<Vec<_>>().join(",");
}

#[test]
fn test() {
	let input = crate::common::get_input(21);
	assert_eq!(part_one(&input), "1815".to_string());
	assert_eq!(
		part_two(&input),
		"kllgt,jrnqx,ljvx,zxstb,gnbxs,mhtc,hfdxb,hbfnkq".to_string()
	);
}
