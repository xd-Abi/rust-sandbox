use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
	println!("{}", "🎲 Welcome to the Number Guessing Game!".bold().cyan());
	println!("{}", "🔢 I have chosen a number between 1 and 100.".yellow());
	println!("{}", "💡 Can you guess what it is? Type a number and press Enter!\n".magenta());

	let random = rand::rng().random_range(1..=100);
	let mut guess: i32;

	loop {
		let mut input = String::new();
		print!("{}", "👉 Your guess: ".bold().blue());
		io::stdin().read_line(&mut input).expect("❌ Failed to read input.");

		match input.trim().parse::<i32>() {
			Ok(num) => guess = num,
			Err(_) => {
				println!("{}", "⚠️ Invalid input! Please enter a valid number.".bright_red());
				continue;
			},
		}

		match guess.cmp(&random) {
			Ordering::Equal => {
				println!(
					"{}",
					format!("🎉 Correct! The number was {}. You win! 🎊", random).green().bold()
				);
				break;
			},
			Ordering::Less => println!("{}", "📈 Too low! Try again.".red()),
			Ordering::Greater => println!("{}", "📉 Too high! Try again.".red()),
		}
	}
}
