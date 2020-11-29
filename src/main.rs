use adventofcode2020::*;

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
	for (day, [one, two]) in all.iter().enumerate() {
	    println!("Day {}:\n\t1: {}\n\t2: {}", day, one(), two());
	}
}
