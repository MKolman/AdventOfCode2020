use wasm_bindgen::prelude::*;

fn solve(input: &Vec<char>, mut i: usize) -> (u64, usize) {
	let mut result = 0_u64;
	let mut op = '+';
	while i < input.len() {
		let (val, di) = match input[i] {
			'(' => solve(input, i + 1),
			')' => return (result, i + 1),
			'+' | '*' => {
				op = input[i];
				i += 1;
				continue;
			}
			' ' => {
				i += 1;
				continue;
			}
			n if n >= '0' && n <= '9' => (n.to_digit(10).unwrap() as u64, i + 1),
			c => panic!(format!("{} is not a valid character", c)),
		};
		i = di;
		if op == '+' {
			result += val;
		} else {
			result *= val;
		}
	}
	return (result, i);
}

fn solve2(input: &Vec<char>, mut i: usize) -> (u64, usize) {
	let mut sums = vec![0];
	let mut op = '+';
	while i < input.len() {
		let (val, di) = match input[i] {
			'(' => solve2(input, i + 1),
			')' => return (sums.iter().fold(1, |a, b| a * b), i + 1),
			'+' | '*' => {
				op = input[i];
				i += 1;
				continue;
			}
			' ' => {
				i += 1;
				continue;
			}
			n if n >= '0' && n <= '9' => (n.to_digit(10).unwrap() as u64, i + 1),
			c => panic!(format!("{} is not a valid character", c)),
		};
		i = di;
		if op == '+' {
			let n = sums.len() - 1;
			sums[n] += val;
		} else {
			sums.push(val);
		}
	}
	return (sums.iter().fold(1, |a, b| a * b), i);
}

#[wasm_bindgen(js_name = day18_part_one)]
pub fn part_one(input: String) -> String {
	let mut result = 0;
	for line in input.lines() {
		result += solve(&line.chars().collect(), 0).0;
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day18_part_two)]
pub fn part_two(input: String) -> String {
	let mut result = 0;
	for line in input.lines() {
		let (val, _) = solve2(&line.chars().collect(), 0);
		result += val;
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(18);
	assert_eq!(part_one(input.clone()), "67800526776934".to_string());
	assert_eq!(part_two(input.clone()), "340789638435483".to_string());
}
