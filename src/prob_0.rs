// Custom sample problem 0 for AOC to test file input!
// this is a program to test reading file input in Rust!

use std::fs;
use color_eyre::eyre::Result;

/// Problem 0 part 1: printing out each line in the file!
pub fn prob_0_1() -> Result<()> {
	let file_path = "input/prob_0.txt";
	let file = fs::read_to_string(file_path)?;

	for line in file.lines() {
		println!("{}", line);
	}

	Ok(())
}

/// Problem 0 part 2: printing out each line, but as one String
/// where a new line character is replaced with a space!
pub fn prob_0_2() -> Result<String> {
	let file_path = "input/prob_0.txt";
	let file = fs::read_to_string(file_path)?;
	
	let mut result = String::new();
	let mut lines: Vec<String> = vec![];
	for line in file.lines() {
		lines.push(line.to_string());
	}

	for x in lines {
		result += &x;
		result.push(' ')
	}

	Ok(result)
}