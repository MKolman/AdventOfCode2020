use std::cmp::{max, min};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

fn get_tiling(input: &str) -> HashSet<(i32, i32)> {
	let mut flipped = HashSet::new();
	for line in input.lines() {
		let (mut x, mut y) = (0, 0);
		let mut instructions = line.chars();
		while let Some(c) = instructions.next() {
			// e, se, sw, w, nw, and ne
			match c {
				'e' => x += 1,
				'w' => x -= 1,
				's' => {
					y -= 1;
					if instructions.next().unwrap() == 'e' {
						x += 1;
					}
				}
				'n' => {
					y += 1;
					if instructions.next().unwrap() == 'w' {
						x -= 1;
					}
				}
				c => panic!(format!("Unknown direction {:?}", c)),
			}
		}
		if flipped.take(&(x, y)) == None {
			flipped.insert((x, y));
		}
	}
	return flipped;
}

#[wasm_bindgen(js_name = day24_part_one)]
pub fn part_one(input: &str) -> String {
	return get_tiling(input).len().to_string();
}

fn count_neighbours(x: i32, y: i32, tiling: &HashSet<(i32, i32)>) -> (bool, usize) {
	let mut count = 0;
	for &(dx, dy) in &[
		(x - 1, y),
		(x + 1, y),
		(x, y + 1),
		(x, y - 1),
		(x + 1, y - 1),
		(x - 1, y + 1),
	] {
		count += tiling.contains(&(dx, dy)) as usize;
	}
	return (tiling.contains(&(x, y)), count);
}

#[wasm_bindgen(js_name = day24_part_two)]
pub fn part_two(input: &str) -> String {
	let mut tiling = get_tiling(input);
	let (mut minx, mut maxx) = (100000, -100000);
	let (mut miny, mut maxy) = (100000, -100000);
	for &(x, y) in &tiling {
		minx = min(minx, x);
		maxx = max(maxx, x);
		miny = min(miny, y);
		maxy = max(maxy, y);
	}
	for _ in 0..100 {
		let tmp = tiling.clone();
		for x in minx - 1..=maxx + 1 {
			for y in miny - 1..=maxy + 1 {
				match count_neighbours(x, y, &tmp) {
					(true, n) if n == 0 || n > 2 => {
						tiling.remove(&(x, y));
					}
					(false, 2) => {
						minx = min(minx, x);
						maxx = max(maxx, x);
						miny = min(miny, y);
						maxy = max(maxy, y);
						tiling.insert((x, y));
					}
					_ => {}
				};
			}
		}
	}
	return tiling.len().to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(24);
	assert_eq!(part_one(&input), "465".to_string());
	assert_eq!(part_two(&input), "4078".to_string());
}
