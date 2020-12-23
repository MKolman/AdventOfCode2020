use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> (Vec<usize>, usize) {
	let mut cups = vec![0_usize; input.len()];
	let mut prev = input.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;
	for c in input.chars() {
		let n = c.to_digit(10).unwrap() as usize - 1;
		cups[prev] = n;
		prev = n;
	}
	let first = cups[prev];
	return (cups, first);
}

fn play(mut cups: Vec<usize>, start: usize, rounds: usize) -> Vec<usize> {
	let mut cur = start;
	let n = cups.len();
	for _ in 0..rounds {
		let next = (cups[cur], cups[cups[cur]], cups[cups[cups[cur]]]);
		cups[cur] = cups[next.2];
		let mut insert_pos = (cur + n - 1) % n;
		while insert_pos == next.0 || insert_pos == next.1 || insert_pos == next.2 {
			insert_pos = (insert_pos + n - 1) % n;
		}
		let tmp = cups[insert_pos];
		cups[insert_pos] = next.0;
		cups[next.2] = tmp;
		cur = cups[cur];
	}
	return cups;
}

#[wasm_bindgen(js_name = day23_part_one)]
pub fn part_one(input: String) -> String {
	let (mut cups, mut cur) = parse_input(&input);
	cups = play(cups, cur, 100);
	let mut result = "".to_string();
	cur = 0;
	loop {
		cur = cups[cur];
		if cur == 0 {
			break;
		}
		result += &(cur + 1).to_string();
	}
	return result;
}

#[wasm_bindgen(js_name = day23_part_two)]
pub fn part_two(input: String) -> String {
	let (mut cups, cur) = parse_input(&input);
	let n = cups.len();
	for v in cups.iter_mut() {
		if *v == cur {
			*v = n;
		}
	}
	cups.resize(1000000, 0);
	for i in n..1000000 {
		cups[i] = i + 1;
	}
	cups[999999] = cur;
	cups = play(cups, cur, 10000000);
	return ((cups[0] + 1) * (cups[cups[0]] + 1)).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(23);
	assert_eq!(part_one(input.clone()), "97624853".to_string());
	assert_eq!(part_two(input.clone()), "664642452305".to_string());
}
