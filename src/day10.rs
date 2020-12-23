use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day10_part_one)]
pub fn part_one(input: &str) -> String {
	let mut num: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
	num.sort_unstable();
	let mut prev = 0;
	let mut cnt = [0, 0, 0];
	for n in &num {
		cnt[n - prev - 1] += 1;
		prev = *n;
	}
	return (cnt[0] * (cnt[2] + 1)).to_string();
}

#[wasm_bindgen(js_name = day10_part_two)]
pub fn part_two(input: &str) -> String {
	let mut num: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
	num.push(0);
	num.sort_unstable();
	fn count(i: usize, num: &[usize], memo: &mut Vec<usize>) -> usize {
		if i >= num.len() - 1 {
			return 1;
		}
		if memo[i] != 0 {
			return memo[i];
		}
		let mut result = 0;
		for j in 1..=3 {
			if i + j >= num.len() || num[i + j] > num[i] + 3 {
				break;
			}
			result += count(i + j, num, memo);
		}
		memo[i] = result;
		return result;
	}
	return count(0, &num, &mut vec![0; num.len()]).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(10);
	assert_eq!(part_one(&input), "2376".to_string());
	assert_eq!(part_two(&input), "129586085429248".to_string());
}
