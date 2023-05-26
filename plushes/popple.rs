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

use crate::traits::visual::{IsSmiling, Visual};

pub enum Popple {
	Blue, Pink, White
}

impl Popple {
	pub fn blue() -> Self {
		Popple::Blue
	}
	pub fn pink() -> Self {
		Popple::Pink
	}
	pub fn white() -> Self {
		Popple::White
	}

	pub fn color(&self) -> &str {
		match self {
			Popple::Blue => "blue",
			Popple::Pink => "pink",
			Popple::White => "white"
		}
	}
}

impl crate::traits::Plushy for Popple {}

impl IsSmiling for Popple {
	fn is(&self) -> Visual {
		Visual::Smiling
	}
}