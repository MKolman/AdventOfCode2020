use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = day09_part_one)]
pub fn part_one(input: String) -> String {
	let nums: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
	for i in 25..nums.len() {
		let mut valid = false;
		'outer: for j in i-25..i {
			for k in j+1..i {
				if nums[j] + nums[k] == nums[i] {
					valid = true;
					break 'outer;
				}
			}
		}
		if !valid { return nums[i].to_string(); }
	}
	return "0".to_string();
}

#[wasm_bindgen(js_name = day09_part_two)]
pub fn part_two(input: String) -> String {
	let nums: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
	let target: i64 = part_one(input).parse().unwrap();
	let (mut start, mut end, mut sum) = (0, 0, 0);
	while sum != target {
		if sum < target {
			sum += nums[end];
			end += 1;
		} else {
			sum -= nums[start];
			start += 1;
		}
	}
	return (nums[start..end].iter().min().unwrap() + nums[start..end].iter().max().unwrap()).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(9);
	assert_eq!(part_one(input.clone()), "542529149".to_string());
	assert_eq!(part_two(input.clone()), "75678618".to_string());
}
