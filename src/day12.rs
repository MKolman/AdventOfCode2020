use std::ops::{AddAssign, Mul, MulAssign};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Complex {
	real: i64,
	imag: i64,
}

impl Complex {
	pub fn new(real: i64, imag: i64) -> Complex {
		Complex {
			real: real,
			imag: imag,
		}
	}
	pub fn rot(deg: i64) -> Complex {
		match deg {
			90 | -270 => Complex::new(0, 1),
			-90 | 270 => Complex::new(0, -1),
			180 | -180 => Complex::new(-1, 0),
			0 | 360 => Complex::new(1, 0),
			_ => panic!(format!("Can't rotate like that: {}", deg)),
		}
	}
}
impl AddAssign<Complex> for Complex {
	fn add_assign(&mut self, another: Complex) {
		self.real += another.real;
		self.imag += another.imag;
	}
}
impl MulAssign<Complex> for Complex {
	fn mul_assign(&mut self, another: Complex) {
		let real = (self.real * another.real) - (self.imag * another.imag);
		self.imag = (self.real * another.imag) + (self.imag * another.real);
		self.real = real;
	}
}
impl Mul<i64> for Complex {
	type Output = Complex;

	fn mul(self, another: i64) -> Complex {
		Complex {
			real: self.real * another,
			imag: self.imag * another,
		}
	}
}

#[wasm_bindgen(js_name = day12_part_one)]
pub fn part_one(input: String) -> String {
	let mut pos = Complex::new(0, 0);
	let mut dir = Complex::new(0, 1);
	for l in input.lines() {
		let c = l.chars().nth(0).unwrap();
		let n: i64 = l[1..].parse().unwrap();
		match c {
			'F' => pos += dir * n,
			'L' => dir *= Complex::rot(-n),
			'R' => dir *= Complex::rot(n),
			'N' => pos += Complex::new(n, 0),
			'S' => pos += Complex::new(-n, 0),
			'E' => pos += Complex::new(0, n),
			'W' => pos += Complex::new(0, -n),
			_ => panic!("Invalid instruction"),
		}
	}
	return (pos.real.abs() + pos.imag.abs()).to_string();
}

#[wasm_bindgen(js_name = day12_part_two)]
pub fn part_two(input: String) -> String {
	let mut pos = Complex::new(0, 0);
	let mut way = Complex::new(1, 10);
	for l in input.lines() {
		let c = l.chars().nth(0).unwrap();
		let n: i64 = l[1..].parse().unwrap();
		match c {
			'F' => pos += way * n,
			'L' => way *= Complex::rot(-n),
			'R' => way *= Complex::rot(n),
			'N' => way += Complex::new(n, 0),
			'S' => way += Complex::new(-n, 0),
			'E' => way += Complex::new(0, n),
			'W' => way += Complex::new(0, -n),
			_ => panic!("Invalid instruction"),
		}
	}
	return (pos.real.abs() + pos.imag.abs()).to_string();
}

#[test]
fn test() {
	let input = crate::common::get_input(12);
	assert_eq!(part_one(input.clone()), "1032".to_string());
	assert_eq!(part_two(input.clone()), "156735".to_string());
}
