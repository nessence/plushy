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
//! Common traits and types for a plush to convey, display or express some behaviour.

use std::{fmt, fmt::Formatter};

pub trait IsSmiling {
	fn is(&self) -> Visual;
}

#[derive(Copy, Clone)]
pub enum Visual {
	Smiling,
	NotSmiling
}

impl std::ops::Not for Visual {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Visual::Smiling => Visual::NotSmiling,
			Visual::NotSmiling => Visual::Smiling
		}
	}
}

impl fmt::Display for Visual {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Visual::Smiling => write!(f, "(:"),
			Visual::NotSmiling => write!(f, "):")
		}
	}
}

// pub enum Happy<T> {
// 	Happy(T),
// 	Unhappy(T)
// }
//
// impl<T> std::ops::Not for Happy<T> {
// 	type Output = Self;
//
// 	fn not(self) -> Self::Output {
// 		match self {
// 			Happy::Happy(t) => Happy::Unhappy(t),
// 			Happy::Unhappy(t) => Happy::Happy(t)
// 		}
// 	}
// }
//
// impl<T: IsSmiling> From<T> for Happy<T> {
// 	fn from(value: T) -> Self {
// 		match value.is() {
// 			Visual::Smiling => Happy::Happy(value),
// 			_ => Happy::Unhappy(value)
// 		}
// 	}
// }