use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> (i64, Vec<&str>) {
	let mut lines = input.lines();
	return (
		lines.next().unwrap().parse().unwrap(),
		lines.next().unwrap().split(',').collect(),
	);
}

#[wasm_bindgen(js_name = day13_part_one)]
pub fn part_one(input: &str) -> String {
	let (time, schedule) = parse_input(input);
	let mut best = (10000000000, 0);
	for bus in schedule {
		if let Ok(num) = bus.parse::<i64>() {
			let wait = num - (time % num);
			if wait < best.0 {
				best = (wait, wait * num);
			}
		}
	}
	return best.1.to_string();
}

fn get_mod_inv(a: i64, m: i64) -> i64 {
	for i in 1..m {
		if (a * i) % m == 1 {
			return i;
		}
	}
	return 0;
}

#[wasm_bindgen(js_name = day13_part_two)]
pub fn part_two(input: &str) -> String {
	let (_, schedule) = parse_input(input);
	let mut m = 1;
	let mut buses = Vec::new();
	for (remain, bus) in schedule.iter().enumerate() {
		if let Ok(num) = bus.parse::<i64>() {
			m *= num;
			buses.push(((num - ((remain as i64) % num)) % num, num));
		}
	}
	let mut result = 0;
	for (remain, modulo) in buses {
		result += remain * m / modulo * get_mod_inv(m / modulo, modulo);
		result %= m;
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(13);
	assert_eq!(part_one(&input), "161".to_string());
	assert_eq!(part_two(&input), "213890632230818".to_string());
}
