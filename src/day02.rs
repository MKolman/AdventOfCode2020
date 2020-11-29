use std::str::FromStr;

fn parse_input(input: &String) -> Vec<i64> {
	input.trim()
		.split(",").map(|s| i64::from_str(s).unwrap())
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

pub fn part_one(input: String) -> String {
	let inp = &mut parse_input(&input);
	inp[1] = 12;
	inp[2] = 2;
	return run_intcode(inp)[0].to_string();
}
pub fn part_two(input: String) -> String {
	let inp = &mut parse_input(&input);
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
	let input = crate::common::get_input(2);
	assert_eq!(part_one(input.clone()), "5110675");
	assert_eq!(part_two(input.clone()), "4847");
}
