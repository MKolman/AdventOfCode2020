use std::io::{Error, ErrorKind};
use std::cmp;

fn parse_input(input: &String) -> Result<Vec<i64>, Error> {
	input.lines()
		.map(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
		.collect()
}

pub fn part_one(input: String) -> String {

	let vec = parse_input(&input).unwrap();
	let result = vec.iter().fold(0, |total, line| total + cmp::max(line/3 - 2, 0));
	return result.to_string();
}

fn calc_fuel(mass: i64) -> i64 {
	let mut result = 0;
	let mut fuel = cmp::max(mass/3 - 2, 0);
	while fuel > 0 {
		result += fuel;
		fuel = cmp::max(fuel/3 - 2, 0);

	}
	return result;
}

pub fn part_two(input: String) -> String {
	let vec = parse_input(&input).unwrap();
	let result = vec.iter().fold(0, |total, mass| total + calc_fuel(*mass));
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(1);
	assert_eq!(part_one(input.clone()), "3320226");
	assert_eq!(part_two(input.clone()), "4977473");
}
