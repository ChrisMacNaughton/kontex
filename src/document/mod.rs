use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
	pub content: String,
	attrs: HashMap<String, String>,
}

impl Document {
	pub fn from_attributes<T: Display + Debug + Eq + Hash>(attrs: HashMap<T, T>)-> Document {
		let mut new_attrs = HashMap::new();
		for (k, v) in &attrs {
			new_attrs.insert(k.to_string(), v.to_string());
		}
		let content = match new_attrs.get("content") {
			Some(s) => s.to_string(),
			None => String::new(),
		};
		let _ = new_attrs.remove("content");
		Document {
			content: content,
			attrs: new_attrs
		}
	}

	pub fn tokens(&self) -> Vec<String> {
		let mut tmp = vec![];
		for word in self.content.split_whitespace() {
			tmp.push(word.into());
		}
		for (k,v) in self.attrs.clone() {
			tmp.push(v);
		}
		tmp
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDocument {
	pub document: Document,
	pub tokens: HashMap<String, usize>
}