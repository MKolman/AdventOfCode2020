use core::cmp::max;
use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<Option<bool>>> {
	input
		.lines()
		.map(|l| {
			l.to_string()
				.trim()
				.chars()
				.map(|c| match c {
					'.' => None,
					'L' => Some(false),
					_ => panic!("Invalid input"),
				})
				.collect()
		})
		.collect()
}

fn count(row: usize, col: usize, seats: &[Vec<Option<bool>>]) -> u8 {
	let mut cnt = 0;
	for (r, srow) in seats.iter().skip(max(row, 1) - 1).take(3).enumerate() {
		for (c, seat) in srow.iter().skip(max(col, 1) - 1).take(3).enumerate() {
			if (r != 1 || c != 1) && *seat == Some(true) {
				cnt += 1;
			}
		}
	}
	return cnt;
}

fn find_neighbours(seats: &[Vec<Option<bool>>]) -> Vec<Vec<Vec<(usize, usize)>>> {
	let mut result = Vec::new();
	for row in 0..seats.len() {
		result.push(Vec::new());
		for col in 0..seats[row].len() {
			result[row].push(Vec::new());
			for dr in -1..=1 {
				for dc in -1..=1 {
					if dr == 0 && dc == 0 {
						continue;
					}
					let mut i = 0;
					loop {
						i += 1;
						let (r, c) = (row as i64 + i * dr, col as i64 + i * dc);
						if r < 0 || c < 0 || r >= seats.len() as i64 || c >= seats[row].len() as i64
						{
							break;
						}
						if seats[r as usize][c as usize] != None {
							result[row][col].push((r as usize, c as usize));
							break;
						}
					}
				}
			}
		}
	}
	return result;
}
fn count_long(seats: &[Vec<Option<bool>>], neighbours: &[(usize, usize)]) -> u8 {
	let mut cnt = 0;
	for (r, c) in neighbours {
		if seats[*r][*c] == Some(true) {
			cnt += 1;
		}
	}
	return cnt;
}

#[wasm_bindgen(js_name = day11_part_one)]
pub fn part_one(input: String) -> String {
	let mut seats = parse_input(&input);
	let mut total_count = 0;
	loop {
		let mut step = seats.clone();
		for r in 0..seats.len() {
			for c in 0..seats[r].len() {
				step[r][c] = match (seats[r][c], count(r, c, &seats)) {
					(Some(false), 0) => {
						total_count += 1;
						Some(true)
					}
					(Some(true), n) if n >= 4 => {
						total_count -= 1;
						Some(false)
					}
					(s, _) => s,
				};
			}
		}
		if step == seats {
			break;
		}
		seats = step;
	}
	return total_count.to_string();
}

#[wasm_bindgen(js_name = day11_part_two)]
pub fn part_two(input: String) -> String {
	let mut seats = parse_input(&input);
	let mut total_count = 0;
	let neighbours = find_neighbours(&seats);
	loop {
		let mut step = seats.clone();
		for r in 0..seats.len() {
			for c in 0..seats[r].len() {
				step[r][c] = match (seats[r][c], count_long(&seats, &neighbours[r][c])) {
					(Some(false), 0) => {
						total_count += 1;
						Some(true)
					}
					(Some(true), n) if n >= 5 => {
						total_count -= 1;
						Some(false)
					}
					(s, _) => s,
				};
			}
		}
		if step == seats {
			break;
		}
		seats = step;
	}
	return total_count.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(11);
	assert_eq!(part_one(input.clone()), "2243".to_string());
	assert_eq!(part_two(input.clone()), "2027".to_string());
}
