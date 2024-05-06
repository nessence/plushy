use plushy::{Plush, Popple};
use plushy::traits::transform::Transform;
use plushy::traits::visual::IsSmiling;
use plushy::greet::Greet;

fn main() {
	let mut plush = Plush::transformable();
	println!("{}", plush.is());

	plush.transform();
	println!("{}", plush.is());

	plush.transform();
	println!("{}", plush.is());

	// Change from cuddle to playSound
	// A Plush can't cuddle ;
	// if it could, the Popple can't cuddle back.
	//
	// What about poking!? Uh, no.
	// Blink?? also creepy
	//
	// TODO: play a sound
	plush.play_sound();

	let popple = Popple::blue();
	popple.greet(&plush);
	plush.greet(&popple);
}