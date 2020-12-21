use std::fs;

pub fn get_input(day: usize) -> String {
	return fs::read_to_string(format!("input/day{:02}.txt", day)).unwrap_or("".to_string());
}
