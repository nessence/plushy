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

	plush.cuddle();

	let popple = Popple::blue();
	popple.greet(&plush);
	plush.greet(&popple);
}