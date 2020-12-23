use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day0325_part_one)]
pub fn part_one(_: &str) -> String {
	return "Not implemented".to_string();
}

#[wasm_bindgen(js_name = day25_part_two)]
pub fn part_two(_: &str) -> String {
	return "Not implemented".to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(25);
	assert_eq!(part_one(&input), "Not implemented".to_string());
	assert_eq!(part_two(&input), "Not implemented".to_string());
}
