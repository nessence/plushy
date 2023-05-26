// Copyright Alex Leverington
// This file is part of Plushy.
// Plushy is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version. Plushy is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
// more details. You should have received a copy of the GNU General Public
// License along with Plushy. If not, see <https://www.gnu.org/licenses/>.

//! Plushes!
//!
//! Common plush traits and types that are interactive and/or interdependent
//!
//! Interdependent crate dependencies create a "chicken-and-egg" problem during
//! packaging because cargo, which is used for building packages, relies on
//! crates.io for dependencies. If two crates depend on each other, neither crate
//! can be packaged because their dependencies are not available for download
//! from crates.io.
//
//! Thus, the greet crate is designated for functionality that depends on
//! multiple crates or requires interdependent types.

//! Implement so a Popple can greet a Plush and a Plush can greet a Popple

use plushy_plushes::{Plush, Popple};
use plushy_plushes::traits::visual::{IsSmiling, Visual};

pub trait Greet<P> {
	fn greet(&self, who: &P);
}

impl Greet<Plush> for Popple {
	fn greet(&self, who: &Plush) {
		let message = match self.is() {
			Visual::Smiling => format!("Hello there Plush! Why are you so {} today?!", who.is().to_string()),
			_ => "...".to_string()
		};
		println!("{}", message);
	}
}

impl Greet<Popple> for Plush {
	fn greet(&self, who: &Popple) {
		let message = match self.is() {
			Visual::Smiling => format!("Hello there Popple! What a nice color {} you are today!", who.color()),
			_ => "...".to_string()
		};
		println!("{}", message);
	}
}