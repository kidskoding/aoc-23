use std::fs;
use color_eyre::eyre::Result;

pub fn prob_1_1() -> Result<u64> {
	let mut sum: u64 = 0;
	let file_path = "input/prob_1.txt";
	let file = fs::read_to_string(file_path)?;

	for line in file.lines() {
		let res = helper(line, true).to_string() + &helper(line, false).to_string();
		sum += res.parse::<u64>()?;
	}

	Ok(sum)
}
fn helper(string: &str, forwards: bool) -> usize {
    let mut iter = string.chars();

    let digit = if forwards {
        iter.find_map(|c| c.to_digit(10).map(|d| d as usize))
    } else {
        iter.rev().find_map(|c| c.to_digit(10).map(|d| d as usize))
    };

    digit.expect("No digit found in the string")
}