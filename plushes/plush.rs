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

use core::cell::Cell;

#[derive(Clone, Copy)]
pub enum Face {
	Smile, Frown
}

impl std::ops::Not for Face {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Face::Smile => Face::Frown,
			_ => Face::Smile
		}
	}
}

/// Some are happy, some are not, and some can transform
pub enum Plush {
	Happy,
	Transformable(Cell<Face>),
	Sombre
}

impl Plush {
	pub fn happy() -> Self {
		Plush::Happy
	}
	pub fn transformable() -> Self {
		Plush::Transformable(Cell::new(Face::Smile))
	}
	pub fn sombre() -> Self {
		Plush::Sombre
	}

	pub fn cuddle(&self) {}
}

impl crate::traits::Plushy for Plush {}

impl crate::traits::visual::IsSmiling for Plush {
	fn is(&self) -> crate::traits::visual::Visual {
		self.into()
	}
}

impl crate::traits::transform::Transform for Plush {
	fn transform(&mut self) {
		if let Plush::Transformable(f) = self {
			f.replace(!f.get());
		}
	}
}