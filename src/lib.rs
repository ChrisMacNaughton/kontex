//! ```toml
//! [dependencies]
//! kontex = "*"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use kontex::{Document, Index, Search};
//! use std::collections::HashMap;
//!
//! let mut index = Index::new();
//!
//! let mut attrs = HashMap::new();
//! attrs.insert("text", "Hello, World!");
//! attrs.insert("title", "Helo");
//! let document = Document::from_attributes(attrs);
//! let _ = index.add_document(document.clone());
//!
//! let mut search: HashMap<&str, &str> = HashMap::new();
//! search.insert("*", "world");
//! let result = index.search(Search::from_attributes(search));
//!
//! println!("{:?}", index);
//! assert_eq!(*result.first().unwrap(), document);
#![crate_name = "kontex"]

mod analyzer;
pub mod analyzers;
mod document;
mod index;
mod search;

pub use document::Document;
pub use index::Index;
pub use search::Search;
// pub use analyzers;
