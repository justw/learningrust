use std::io;

fn main() {

	println!("Take a guess:");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.ok()
		.expect("Failed to read line");

	println!("You guessed: {}", guess);

}