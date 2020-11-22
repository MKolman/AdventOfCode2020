use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


fn get_input() -> Vec<i64> {
    let br = BufReader::new(File::open("input/day02.txt").unwrap());
    br.lines().next().unwrap().unwrap().split(",").map(|s| i64::from_str(s).unwrap())
        .collect()
}

fn run_intcode(mut code: Vec<i64>) -> Vec<i64> {
	let mut i = 0;
	loop {
		match code[i] {
			99 => break,
			1 | 2 => {
				let idx = code[i+3] as usize;
				match code[i] {
					1 => code[idx] = code[code[i+1] as usize] + code[code[i+2] as usize],
					2 => code[idx] = code[code[i+1] as usize] * code[code[i+2] as usize],
					_ => panic!("What??"),
				}
				i += 4;
			},
			_ => panic!("Unknown intcode"),
		}
	}
	return code;
}

pub fn part_one() -> i64 {
	let mut inp = get_input();
	inp[1] = 12;
	inp[2] = 2;
	inp = run_intcode(inp);
	return inp[0];
}
pub fn part_two() -> i64 {
	let mut inp = get_input();
	for result in 0..9999 {
		inp[1] = result / 100;
		inp[2] = result % 100;
		if run_intcode(inp.clone())[0] == 19690720 {
			return result;
		}
	}
	return 0;
}

#[test]
fn test() {
	assert_eq!(part_one(), 5110675);
	assert_eq!(part_two(), 4847);
}
