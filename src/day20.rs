use std::collections::HashSet;
use wasm_bindgen::prelude::*;

type BorderHash = usize;


const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;

struct Tile {
	id: usize,
	size: usize,
	val: Vec<Vec<bool>>,
	borders: [BorderHash; 4],
	neighbours: [Option<usize>; 4]
}
impl Tile {
	fn new(id: usize, val: Vec<Vec<bool>>) -> Tile {
		let borders = hash_borders(&val);
		Tile{
			id: id,
			size: val.len(),
			val: val,
			borders: borders,
			neighbours: [None; 4]
		}
	}

	fn flip(&mut self) {
		self.neighbours.swap(E, W);
		for i in 0..self.size {
			self.val[i].reverse();
		}
		self.borders = hash_borders(&self.val);
	}

	fn rotate(&mut self) {
		let tmp = self.neighbours[N];
		self.neighbours[N] = self.neighbours[W];
		self.neighbours[W] = self.neighbours[S];
		self.neighbours[S] = self.neighbours[E];
		self.neighbours[E] = tmp;
		self.val = rotate(&self.val);
		self.borders = hash_borders(&self.val);
	}

	fn no_border(&self) -> Vec<Vec<bool>> {
		return self.val[1..=8].iter().map(|r| r[1..=8].iter().map(|c| *c).collect()).collect();
	}
}

fn rotate(v: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut result = v.clone();
	for (r, row) in v.iter().enumerate() {
		for (c, cell) in row.iter().enumerate() {
			result[c][v.len() - 1 - r] = *cell;
		}
	}
	return result;
}

fn parse_input(input: &String) -> Vec<Tile> {
	let result = input.trim().split("\n\n").map(|tile| {
		let mut lines = tile.lines();
		let title = lines.next().unwrap();
		let id = title[5..title.len()-1].parse().unwrap();
		let map = lines.map(|line| line.chars().map(|c| c == '#').collect()).collect();
		return Tile::new(id, map);
	}).collect();
	return find_neighbours(result);
}

fn find_neighbours(mut tiles: Vec<Tile>) -> Vec<Tile> {
	let mut visited = HashSet::new();
	visited.insert(0);
	let mut stack = vec![0];
	while stack.len() > 0 {
		let id = stack.pop().unwrap();
		'side: for side in 0..4 {
			let border = tiles[id].borders[side];
			let opposite = (side + 2) % 4;
			for i in 0..tiles.len() {
				if i == id { continue; }
				for _ in 0..2 {
					for _ in 0..4 {
						if tiles[i].borders[opposite] == border {
							if !visited.contains(&i) {
								stack.push(i);
								visited.insert(i);
							}
							tiles[id].neighbours[side] = Some(i);
							tiles[i].neighbours[opposite] = Some(id);
							continue 'side;
						}
						tiles[i].rotate();
					}
					tiles[i].flip();
				}
			}
		}
	}
	return tiles;
}

fn hash_one(border: &Vec<bool>) -> BorderHash {
	let mut hash = 0;
	for v in border {
		hash = hash*2 + *v as usize;
	}
	return hash;
}

fn hash_borders(tile: &Vec<Vec<bool>>) -> [BorderHash; 4] {
	let (w, h) = (tile[0].len(), tile.len());
	let mut result = [0; 4];
	result[N] = hash_one(&tile[0]);
	result[W] = hash_one(&tile.iter().map(|row| row[0]).collect());
	result[E] = hash_one(&tile.iter().map(|row| row[w-1]).collect());
	result[S] = hash_one(&tile[h-1]);
	return result;
}

#[wasm_bindgen(js_name = day20_part_one)]
pub fn part_one(input: String) -> String {
	let tiles = parse_input(&input);
	let mut result = 1;
	for tile in tiles {
		if tile.neighbours.iter().map(|n| (*n == None) as usize).sum::<usize>() == 2 {
			result *= tile.id;
		}
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day20_part_two)]
pub fn part_two(input: String) -> String {
	let tiles = parse_input(&input);
	let mut start = 0;
	for (i, tile) in tiles.iter().enumerate() {
		if tile.neighbours[N] == None && tile.neighbours[W] == None {
			start = i;
		}
	}
	let mut grid = vec![vec![start]];
	let mut full = tiles[start].no_border();
	let height = full.len();
	loop {
		let row = grid.len() - 1;
		if let Some(n) = tiles[*grid[row].last().unwrap()].neighbours[E] {
			grid.last_mut().unwrap().push(n);
			for (i, r) in tiles[n].no_border().iter_mut().enumerate() {
				full[i+row*height].append(r);
			}
		} else if let Some(n) = tiles[grid[row][0]].neighbours[S] {
			grid.push(vec![n]);
			full.append(&mut tiles[n].no_border());
		} else {
			break;
		}
	}
	let mut count_monsters = 0;
	let monster: Vec<Vec<char>> = vec![
		"                  # ".chars().collect(),
		"#    ##    ##    ###".chars().collect(),
		" #  #  #  #  #  #   ".chars().collect(),
	];
	'outer: for _ in 0..2 {
		for _ in 0..4 {
			for row in 0..full.len()-monster.len() {
				'start: for col in 0..full[0].len()-monster[0].len() {
					for r in 0..monster.len() {
						for c in 0..monster[0].len() {
							if monster[r][c] == '#' && !full[row+r][col+c] {
								continue 'start;
							}
						}
					}
					count_monsters += 1;
				}
			}
			if count_monsters > 0 { break 'outer; }
			full = rotate(&full);
		}
		for row in full.iter_mut() {
			row.reverse();
		}
	}
	let all_hash: usize = full.iter().map(|row| row.iter().map(|cell| *cell as usize).sum::<usize>()).sum();
	return (all_hash - count_monsters*15).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(20);
	assert_eq!(part_one(input.clone()), "15405893262491".to_string());
	assert_eq!(part_two(input.clone()), "2133".to_string());
}
