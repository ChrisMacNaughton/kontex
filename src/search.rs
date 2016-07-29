use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Search {
    pub options: HashMap<String, String>
}

impl Search {
    pub fn from_attributes<T: Display + Debug + Eq + Hash>(attrs: HashMap<T, T>) -> Search {
         let mut new_attrs = HashMap::new();
        for (k, v) in &attrs {
            new_attrs.insert(k.to_string(), v.to_string());
        }
        Search {
            options: new_attrs,
        }
    }
}
