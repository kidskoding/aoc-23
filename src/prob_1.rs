use std::{collections::HashMap, fs};
use color_eyre::eyre::Result;
use regex::Regex;

pub fn prob_1_1() -> Result<u64> {
	let mut sum: u64 = 0;
	let file_path = "input/prob_1_sample.txt";
	let file = fs::read_to_string(file_path)?;

	for line in file.lines() {
		let digit_from_front = line.chars()
			.find_map(|c| c.to_digit(10).map(|x| x as usize))
			.unwrap();
		let digit_from_back = line.chars()
			.rev()
			.find_map(|c| c.to_digit(10).map(|x| x as usize))
			.unwrap();

		let res = format!("{}{}", digit_from_front, digit_from_back);
		sum += res.parse::<u64>()?;
	}

	Ok(sum)
}

pub fn prob_1_2() -> Result<u64> {
	let mut sum: u64 = 0;
	let file_path = "input/prob_1_2_sample.txt";
	let file = fs::read_to_string(file_path)?;

	let map: HashMap<&str, u32> = HashMap::from([
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9)
	]);
	let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")?;

	for line in file.lines() {
		let mut updated_line = line.to_string();
		for capture in re.captures_iter(line) {
			if let Some(matched_word) = capture.get(0) {
				updated_line = line.replace(matched_word.as_str(), &map.get(matched_word.as_str())
					.unwrap()
					.to_string());
			}
        }

        let digit_from_front = updated_line.chars()
			.find_map(|c| c.to_digit(10).map(|x| x as usize))
			.unwrap();
		let digit_from_back = updated_line.chars()
			.rev()
			.find_map(|c| c.to_digit(10).map(|x| x as usize))
			.unwrap();

		let res = format!("{}{}", digit_from_front, digit_from_back);
		sum += res.parse::<u64>()?;
	}

	Ok(sum)
}
