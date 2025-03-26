use aoc_2023::{prob_0::{prob_0_1, prob_0_2}, prob_1::{prob_1_1, prob_1_2}};
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Problem 0 pt 1: "); prob_0_1()?;
    println!("Problem 0 pt 2: {}\n", prob_0_2()?);

    println!("Problem 1 pt 1: {}", prob_1_1()?);
    println!("Problem 1 pt 2: {}\n", prob_1_2()?);

    println!("Problem 2 pt 1: "); 
    println!("Problem 2 pt 2: \n");

    println!("Problem 3 pt 1: "); 
    println!("Problem 3 pt 2: \n");

    println!("Problem 4 pt 1: "); 
    println!("Problem 4 pt 2: \n");

    println!("Problem 5 pt 1: "); 
    println!("Problem 5 pt 2: ");

    Ok(())
}
