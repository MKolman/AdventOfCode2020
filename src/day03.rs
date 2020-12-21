use wasm_bindgen::prelude::*;

fn parse_input(input: &String) -> Vec<Vec<bool>> {
	input
		.lines()
		.map(|s| s.chars().map(|c| c == '#').collect())
		.collect()
}

#[wasm_bindgen(js_name = day03_part_one)]
pub fn part_one(input: String) -> String {
	let inp = parse_input(&input);
	let mut result = 0;
	for (i, line) in inp.iter().enumerate() {
		result += line[(3 * i) % line.len()] as i64;
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day03_part_two)]
pub fn part_two(input: String) -> String {
	let inp = parse_input(&input);
	let mut result = [0, 0, 0, 0, 0];
	for (i, line) in inp.iter().enumerate() {
		for (j, slope) in [1, 3, 5, 7].iter().enumerate() {
			result[j] += line[(slope * i) % line.len()] as i64;
		}
		if i % 2 == 0 {
			result[4] += line[(i / 2) % line.len()] as i64;
		}
	}
	return result.iter().fold(1, |total, r| total * r).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(3);
	assert_eq!(part_one(input.clone()), "240");
	assert_eq!(part_two(input.clone()), "2832009600");
}
