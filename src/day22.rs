use std::collections::HashSet;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

fn parse_input(input: &str) -> Vec<VecDeque<usize>> {
	input
		.split("\n\n")
		.map(|deck| {
			let mut lines = deck.lines();
			lines.next();
			lines.map(|n| n.parse().unwrap()).collect()
		})
		.collect()
}

#[wasm_bindgen(js_name = day22_part_one)]
pub fn part_one(input: &str) -> String {
	let mut decks = parse_input(&input);
	while !decks[0].is_empty() && !decks[1].is_empty() {
		let (a, b) = (decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap());
		if a > b {
			decks[0].push_back(a);
			decks[0].push_back(b);
		} else {
			decks[1].push_back(b);
			decks[1].push_back(a);
		}
	}
	let mut result = 0;
	for d in &decks {
		for (i, n) in d.iter().enumerate() {
			result += (d.len() - i) * n;
		}
	}
	return result.to_string();
}

fn play(mut decks: Vec<VecDeque<usize>>, top_level: bool) -> usize {
	let mut memo = HashSet::new();
	while !decks[0].is_empty() && !decks[1].is_empty() {
		let state = decks.clone();
		if memo.contains(&state) {
			assert!(
				!top_level,
				"Oops, I assumed the top-level game can't end in recursion."
			);
			return 0;
		}
		memo.insert(state);
		// Get top cards
		let (a, b) = (decks[0].pop_front().unwrap(), decks[1].pop_front().unwrap());
		if decks[0].len() < a || decks[1].len() < b {
			// If not enough cards in the deck just compare their values
			if a > b {
				decks[0].push_back(a);
				decks[0].push_back(b);
			} else {
				decks[1].push_back(b);
				decks[1].push_back(a);
			}
		} else {
			// Play sub-game
			let winner = play(
				vec![
					decks[0].iter().copied().take(a).collect(),
					decks[1].iter().copied().take(b).collect(),
				],
				false,
			);
			if winner == 0 {
				decks[0].push_back(a);
				decks[0].push_back(b);
			} else {
				decks[1].push_back(b);
				decks[1].push_back(a);
			}
		}
	}

	// For top level game return score, otherwise return winner
	if top_level {
		decks
			.iter()
			.map(|d| {
				d.iter()
					.enumerate()
					.map(|(i, n)| (d.len() - i) * n)
					.sum::<usize>()
			})
			.sum()
	} else if decks[0].is_empty() {
		return 1;
	} else {
		return 0;
	}
}

#[wasm_bindgen(js_name = day22_part_two)]
pub fn part_two(input: &str) -> String {
	let decks = parse_input(&input);
	let result = play(decks, true);
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(22);
	assert_eq!(part_one(&input), "32033".to_string());
	assert_eq!(part_two(&input), "34901".to_string());
}
