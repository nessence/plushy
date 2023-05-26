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

use crate::traits::visual::Visual;
use crate::{Plush, Face, Popple};

impl From<Face> for Visual {
	fn from(value: Face) -> Self {
		match value {
			Face::Smile => Visual::Smiling,
			Face::Frown => Visual::NotSmiling
		}
	}
}

impl From<&Plush> for Visual {
	fn from(value: &Plush) -> Self {
		match value {
			Plush::Happy => Visual::Smiling,
			Plush::Transformable(h) => h.get().into(),
			_ => Visual::NotSmiling
		}
	}
}

impl From<&Popple> for Visual {
	fn from(_value: &Popple) -> Self {
		Visual::Smiling
	}
}