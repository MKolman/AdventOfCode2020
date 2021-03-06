use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
	return input
		.lines()
		.map(|line| line.chars().map(|c| c == '#').collect())
		.collect();
}

fn count_neighbours(x: usize, y: usize, z: usize, field: &[Vec<Vec<bool>>]) -> usize {
	let mut result = 0;
	for plane in field.iter().skip(z - 1).take(3) {
		for line in plane.iter().skip(y - 1).take(3) {
			for cell in line.iter().skip(x - 1).take(3) {
				result += *cell as usize;
			}
		}
	}
	return result;
}
fn count_neighbours4(
	x: usize,
	y: usize,
	z: usize,
	w: usize,
	field: &[Vec<Vec<Vec<bool>>>],
) -> usize {
	let mut result = 0;
	for space in field.iter().skip(w - 1).take(3) {
		result += count_neighbours(x, y, z, space);
	}
	return result;
}

#[wasm_bindgen(js_name = day17_part_one)]
pub fn part_one(input: &str) -> String {
	let start = parse_input(&input);
	let mut total = vec![vec![vec![false; start[0].len() + 14]; start.len() + 14]; 15];
	for (i, line) in start.iter().enumerate() {
		for (j, cell) in line.iter().enumerate() {
			total[7][i + 7][j + 7] = *cell;
		}
	}
	for _ in 0..6 {
		let mut step = total.clone();
		for z in 1..total.len() - 1 {
			for y in 1..total[0].len() - 1 {
				for x in 1..total[0][0].len() - 1 {
					let neighbours = count_neighbours(x, y, z, &total);
					if total[z][y][x] && (neighbours < 3 || neighbours > 4) {
						step[z][y][x] = false;
					} else if !total[z][y][x] && neighbours == 3 {
						step[z][y][x] = true;
					}
				}
			}
		}
		total = step;
	}
	let mut result = 0;
	for plane in total {
		for line in plane {
			for cell in line {
				result += cell as usize;
			}
		}
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day17_part_two)]
pub fn part_two(input: &str) -> String {
	let start = parse_input(&input);
	let mut total = vec![vec![vec![vec![false; start[0].len() + 14]; start.len() + 14]; 15]; 15];
	for (i, line) in start.iter().enumerate() {
		for (j, cell) in line.iter().enumerate() {
			total[7][7][i + 7][j + 7] = *cell;
		}
	}
	for _ in 0..6 {
		let mut step = total.clone();
		for w in 1..total.len() - 1 {
			for z in 1..total[0].len() - 1 {
				for y in 1..total[0][0].len() - 1 {
					for x in 1..total[0][0][0].len() - 1 {
						let neighbours = count_neighbours4(x, y, z, w, &total);
						if total[w][z][y][x] && (neighbours < 3 || neighbours > 4) {
							step[w][z][y][x] = false;
						} else if !total[w][z][y][x] && neighbours == 3 {
							step[w][z][y][x] = true;
						}
					}
				}
			}
		}
		total = step;
	}
	let mut result = 0;
	for space in total {
		for plane in space {
			for line in plane {
				for cell in line {
					result += cell as usize;
				}
			}
		}
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(17);
	assert_eq!(part_one(&input), "362".to_string());
	assert_eq!(part_two(&input), "1980".to_string());
}
