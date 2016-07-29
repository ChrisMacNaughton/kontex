use std::collections::HashMap;

use super::super::analyzer::Analyzer;
use super::super::document::{Document, ParsedDocument};

#[derive(Debug)]
pub struct StandardAnalyzer {
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use super::*;
	use super::super::super::analyzer::Analyzer;
	use super::super::super::document::{Document, ParsedDocument};

	#[test]
	fn it_parses_a_document() {
		let mut attrs = HashMap::new();
		attrs.insert("content", "hello");
		let document = Document::from_attributes(attrs);
		let mut expected_tokens = HashMap::new();
		expected_tokens.insert("hello".to_string(), 1);

		let expected = ParsedDocument {
			document: document.clone(),
			tokens: expected_tokens,
		};

		assert_eq!(StandardAnalyzer::new().parse(document), expected);
	}
}

impl Analyzer for StandardAnalyzer {
	fn new() -> StandardAnalyzer {
		StandardAnalyzer{}
	}

	fn parse(&self, doc: Document) -> ParsedDocument {
		let mut words: Vec<String> = vec![];
		for word in doc.tokens() {
			words.push(word.into());
		}

		let mut word_count: HashMap<String, usize> = HashMap::new();
    
	    for word in words {
	        let mut count = word_count.entry(word).or_insert(0);
	        *count += 1;
	    }

		ParsedDocument {
			document: doc,
			tokens: word_count,
		}
	}
}