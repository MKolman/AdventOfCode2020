#![allow(clippy::needless_return)]

use adventofcode2020::*;
use std::time::Instant;

fn main() {
	let all = [
		[day01::part_one, day01::part_two],
		[day02::part_one, day02::part_two],
		[day03::part_one, day03::part_two],
		[day04::part_one, day04::part_two],
		[day05::part_one, day05::part_two],
		[day06::part_one, day06::part_two],
		[day07::part_one, day07::part_two],
		[day08::part_one, day08::part_two],
		[day09::part_one, day09::part_two],
		[day10::part_one, day10::part_two],
		[day11::part_one, day11::part_two],
		[day12::part_one, day12::part_two],
		[day13::part_one, day13::part_two],
		[day14::part_one, day14::part_two],
		[day15::part_one, day15::part_two],
		[day16::part_one, day16::part_two],
		[day17::part_one, day17::part_two],
		[day18::part_one, day18::part_two],
		[day19::part_one, day19::part_two],
		[day20::part_one, day20::part_two],
		[day21::part_one, day21::part_two],
		[day22::part_one, day22::part_two],
		[day23::part_one, day23::part_two],
		[day24::part_one, day24::part_two],
		[day25::part_one, day25::part_two],
	];
	let start = Instant::now();
	for (day, [one, two]) in all.iter().enumerate() {
		let input = common::get_input(day + 1);
		println!("Day {}:", day + 1);
		let start_one = Instant::now();
		let sol_one = one(&input);
		let dur_one = start_one.elapsed();
		println!("\t1: {} ({:?})", sol_one, dur_one);
		let start_two = Instant::now();
		let sol_two = two(&input);
		let dur_two = start_two.elapsed();
		println!("\t2: {} ({:?})", sol_two, dur_two);
	}
	println!("Total time: {:?}", start.elapsed());
}
