use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Search {}

impl Search {
	pub fn from_attributes<T: Display + Debug + Eq + Hash>(attrs: HashMap<T, T>) -> Search {
		Search {}
	}
}