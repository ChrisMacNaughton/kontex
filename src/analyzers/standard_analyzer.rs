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
        let mut tokens = HashMap::new();
        tokens.insert("content".to_string(), expected_tokens);

        let expected = ParsedDocument {
            document: document.clone(),
            tokens: tokens,
        };

        assert_eq!(StandardAnalyzer::new().parse(document), expected);
    }
}


impl StandardAnalyzer {
    fn escape(body: String) -> String {
        let mut body = body;
        body = body.chars().filter(|ch| ch.is_alphanumeric()).collect();
        body
    }
}

impl Analyzer for StandardAnalyzer {
    fn new() -> StandardAnalyzer {
        StandardAnalyzer {}
    }

    fn tokenize(body: &String) -> Vec<String> {
        body.split_whitespace()
            .map(|s| s.to_lowercase())
            .map(|s| StandardAnalyzer::escape(s))
            .collect()
    }

    fn tokenize_string(&self, body: &String) -> Vec<String> {
        StandardAnalyzer::tokenize(body)
    }

    fn parse(&self, doc: Document) -> ParsedDocument {
        let tokens = doc.tokenize(&StandardAnalyzer::tokenize);
        // println!("{:?}",tokens);
        let mut full_word_count: HashMap<String, HashMap<String, usize>> = HashMap::new();

        for (ref key, ref tokens) in tokens {
            let mut word_count: HashMap<String, usize> = HashMap::new();

            for word in tokens {
                let mut count = word_count.entry(word.to_owned()).or_insert(0);
                *count += 1;
            }
            full_word_count.insert(key.to_string(), word_count);
        }
        // let mut word_count: HashMap<String, usize> = HashMap::new();

        // for word in words {
        //     let mut count = word_count.entry(word).or_insert(0);
        //     *count += 1;
        // }

        // let mut extra_tokens = HashMap::new();



        ParsedDocument {
            document: doc,
            tokens: full_word_count, // extra_tokens:
        }
    }
}
