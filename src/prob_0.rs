// this is a file to test reading file input in Rust!

use std::fs;
use color_eyre::eyre::Result;

pub fn prob_0() -> Result<()> {
	let file_path = "input/prob_0.txt";
	let file = fs::read_to_string(file_path)?;

	for line in file.lines() {
		println!("{}", line);
	}

	Ok(())
}
