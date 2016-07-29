use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    attrs: HashMap<String, String>,
}

impl Document {
    pub fn from_attributes<T: Display + Debug + Eq + Hash>(attrs: HashMap<T, T>) -> Document {
        let mut new_attrs = HashMap::new();
        for (k, v) in &attrs {
            new_attrs.insert(k.to_string(), v.to_string());
        }
        Document { attrs: new_attrs }
    }

    pub fn tokenize<T: Fn(&String) -> Vec<String>>(&self,
                                                   func: &T)
                                                   -> HashMap<String, Vec<String>> {
        let mut parts = HashMap::new();
        for (key, body) in &self.attrs {
            parts.insert(key.to_owned(), (func)(body));
        }
        parts
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDocument {
    pub document: Document,
    pub tokens: HashMap<String, HashMap<String, usize>>,
}
