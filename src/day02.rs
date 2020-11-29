use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


fn get_input() -> Vec<i64> {
    let br = BufReader::new(File::open("input/day02.txt").unwrap());
    br.lines().next().unwrap().unwrap().split(",").map(|s| i64::from_str(s).unwrap())
        .collect()
}

fn run_intcode(code: & mut Vec<i64>) -> &mut Vec<i64> {
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

pub fn part_one() -> String {
	let inp = &mut get_input();
	inp[1] = 12;
	inp[2] = 2;
	return run_intcode(inp)[0].to_string();
}
pub fn part_two() -> String {
	let inp = &mut get_input();
	for result in 0..9999 {
		inp[1] = result / 100;
		inp[2] = result % 100;
		if run_intcode(&mut inp.clone())[0] == 19690720 {
			return result.to_string();
		}
	}
	return "0".to_string();
}

#[test]
fn test() {
	assert_eq!(part_one(), "5110675");
	assert_eq!(part_two(), "4847");
}
