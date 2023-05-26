use plushy_plushes::Popple;
use plushy_traits::visual::{IsSmiling, Visual};

use std::matches;

#[test]
fn test_popple_is_happy() {
	let popple = Popple::blue();
	assert!(matches!(popple.is(), Visual::Smiling));
}