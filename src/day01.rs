use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::cmp;


fn get_input() -> Result<Vec<i64>, Error> {
    let br = BufReader::new(File::open("input/day01.txt")?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn part_one() -> i64 {
	let vec = get_input().unwrap();
	let result = vec.iter().fold(0, |total, line| total + cmp::max(line/3 - 2, 0));
	return result;
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

pub fn part_two() -> i64 {
	let vec = get_input().unwrap();
	let result = vec.iter().fold(0, |total, mass| total + calc_fuel(*mass));
	return result;
}

#[test]
fn test() {
	assert_eq!(part_one(), 3320226);
	assert_eq!(part_two(), 4977473);
}
