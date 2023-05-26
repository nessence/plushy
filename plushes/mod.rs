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
//! Usable example, demonstration and equivalent for `plushy`.
//! Please see `plushy` for additional documentation.
//!
//! Implements `Plush`, exports all other `plushy` crates and is
//! subsequently used and re-exported by `plushy`.

pub extern crate plushy_traits as traits;

mod plush;
pub use plush::{Plush, Face};

mod popple;
pub use popple::Popple;

mod ext;