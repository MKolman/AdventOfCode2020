use wasm_bindgen::prelude::*;

fn parse_input(input: &String) -> Vec<usize> {
	input
		.lines()
		.map(|line| {
			let bin = line
				.replace("B", "1")
				.replace("F", "0")
				.replace("R", "1")
				.replace("L", "0");
			usize::from_str_radix(&bin, 2).unwrap()
		})
		.collect()
}

#[wasm_bindgen(js_name = day05_part_one)]
pub fn part_one(input: String) -> String {
	let seats = parse_input(&input);
	return seats.iter().max().unwrap().to_string();
}

#[wasm_bindgen(js_name = day05_part_two)]
pub fn part_two(input: String) -> String {
	let mut seats = parse_input(&input);
	seats.sort();
	for (i, id) in seats.iter().enumerate() {
		if i == 0 {
			continue;
		};
		if seats[i - 1] == id - 2 {
			return (id - 1).to_string();
		}
	}
	return "0".to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(5);
	assert_eq!(part_one(input.clone()), "885".to_string());
	assert_eq!(part_two(input.clone()), "623".to_string());
}
