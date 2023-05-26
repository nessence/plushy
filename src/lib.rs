//! Plushes!
//!
//! A plush is a composition of traits... Observable behaviour, internal action
//! and external interactions are defined as a module in a trait, view or interaction crate.
//! Future features could enable plushes to be scripted in order to perform in a play.
//! For such performances, printing a playbill for attending audience would be obligatory.
//!
//! Plush, an example and usable implementation, can [`transform()`] and, if smiling, [`is()`]
//! [`Visual::Smiling`].
//!
//! This crate is a demonstration (and eventually a guide) for implementing a crate.

pub use plushy_plushes::{Plush, Popple, traits};
pub use plushy_greet as greet;