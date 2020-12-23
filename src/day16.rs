use regex::Regex;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

type Rule = (String, [u64; 4]);
type Ticket = Vec<u64>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Ticket>) {
	let groups: Vec<&str> = input.splitn(2, "\n\n").collect();
	let mut rules = Vec::new();
	let rule_re = Regex::new("([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
	for rule in rule_re.captures_iter(groups[0]) {
		// println!("{:?}", rule);
		rules.push((
			rule[1].to_string(),
			[
				rule[2].parse().unwrap(),
				rule[3].parse().unwrap(),
				rule[4].parse().unwrap(),
				rule[5].parse().unwrap(),
			],
		));
	}

	let ticket_re = Regex::new("([0-9]+,)+").unwrap();
	let mut tickets = Vec::new();
	for ticket in groups[1].lines() {
		if ticket_re.is_match(ticket) {
			tickets.push(ticket.split(',').map(|s| s.parse().unwrap()).collect());
		}
	}
	return (rules, tickets);
}

#[wasm_bindgen(js_name = day16_part_one)]
pub fn part_one(input: &str) -> String {
	let (rules, tickets) = parse_input(&input);
	let mut result = 0;
	for ticket in &tickets[1..] {
		'inner: for val in ticket {
			for (_, [lo, hi, lo2, hi2]) in &rules {
				if *lo <= *val && *val <= *hi || *lo2 <= *val && *val <= *hi2 {
					continue 'inner;
				}
			}
			result += val;
		}
	}
	return result.to_string();
}

#[wasm_bindgen(js_name = day16_part_two)]
pub fn part_two(input: &str) -> String {
	let (rules, tickets) = parse_input(&input);
	let mut valid_tickets = Vec::new();
	'outer: for ticket in &tickets {
		'inner: for val in ticket {
			for (_, [lo, hi, lo2, hi2]) in &rules {
				if *lo <= *val && *val <= *hi || *lo2 <= *val && *val <= *hi2 {
					continue 'inner;
				}
			}
			continue 'outer;
		}
		valid_tickets.push(ticket);
	}
	let size = tickets[0].len();
	let nrule = rules.len();
	let mut rule2col = Vec::new();
	for i in 0..nrule {
		rule2col.push(HashSet::new());
		for col in 0..size {
			rule2col[i].insert(col);
		}
	}
	for (i, (_, [lo, hi, lo2, hi2])) in rules.iter().enumerate() {
		'cols: for column in 0..size {
			for ticket in &valid_tickets {
				if !(*lo <= ticket[column] && ticket[column] <= *hi
					|| *lo2 <= ticket[column] && ticket[column] <= *hi2)
				{
					rule2col[i].remove(&column);
					continue 'cols;
				}
			}
		}
	}

	let mut change = true;
	while change {
		change = false;
		for i in 0..nrule {
			if rule2col[i].len() == 1 {
				let col = *rule2col[i].iter().next().unwrap();
				for (j, cols) in rule2col.iter_mut().enumerate() {
					if i != j && cols.contains(&col) {
						cols.remove(&col);
						change = true;
					}
				}
			}
		}
	}

	let mut result = 1;
	for (i, (rname, _)) in rules.iter().enumerate() {
		if rname.contains("departure") {
			let col = *rule2col[i].iter().next().unwrap();
			result *= tickets[0][col];
		}
	}
	return result.to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(16);
	assert_eq!(part_one(&input), "22073".to_string());
	assert_eq!(part_two(&input), "1346570764607".to_string());
}
