use wasm_bindgen::prelude::*;

fn parse_input(input: &String) -> Vec<(i64, i64, char, String)> {
	input.lines()
		.map(|s| {
			let parts: Vec<&str> = s.split(' ').collect();
			let limits: Vec<&str> = parts[0].split('-').collect();
			return (
				limits[0].parse().unwrap(),
				limits[1].parse().unwrap(),
				parts[1].chars().nth(0).unwrap(),
				parts[2].to_string()
			);
		})
		.collect()
}

#[wasm_bindgen(js_name = day02_part_one)]
pub fn part_one(input: String) -> String {
	let inp = &parse_input(&input);
	let mut result = 0;
	for (min, max, c, pass) in inp {
		let cnt = pass.matches(&c.to_string()).count() as i64;
		result += (min <= &cnt && max >= &cnt) as i64;
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day02_part_two)]
pub fn part_two(input: String) -> String {
	let inp = &parse_input(&input);
	let mut result = 0;
	for (min, max, c, pass) in inp {
		result += (
			(pass.chars().nth((min-1) as usize) == Some(*c)) ^
			(pass.chars().nth((max-1) as usize) == Some(*c))
		) as i64;
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(2);
	assert_eq!(part_one(input.clone()), "517");
	assert_eq!(part_two(input.clone()), "284");
}
