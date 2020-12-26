use wasm_bindgen::prelude::*;

const MOD: i64 = 20201227;
#[wasm_bindgen(js_name = day0325_part_one)]
pub fn part_one(input: &str) -> String {
	let public: Vec<i64> = input.lines().map(|l| l.parse::<i64>().unwrap()).collect();
	let mut result = 1;
	let mut prod = 1;
	while prod != public[0] {
		prod = (prod * 7) % MOD;
		result = (result * public[1]) % MOD;
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day25_part_two)]
pub fn part_two(_: &str) -> String {
	return "Done".to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(25);
	assert_eq!(part_one(&input), "19414467".to_string());
	assert_eq!(part_two(&input), "Done".to_string());
}
