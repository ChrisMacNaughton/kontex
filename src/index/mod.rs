use std::collections::BTreeMap;
use std::path::PathBuf;

use super::analyzer::Analyzer;
use super::analyzers::standard_analyzer::StandardAnalyzer;
use super::document::{Document, ParsedDocument};
use super::Search;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{Document, Search};
    use std::collections::HashMap;
    #[test]
    fn it_queries() {
        let mut index = Index::new();

        let mut attrs = HashMap::new();
        attrs.insert("text", "Hello, World!");
        attrs.insert("title", "Helo");
        let document = Document::from_attributes(attrs);
        index.add_document(document.clone());

        let mut search: HashMap<&str, &str> = HashMap::new();
        search.insert("*", "world");
        let result = index.search(Search::from_attributes(search));

        assert_eq!(*result.first().unwrap(), document);
    }
}

#[derive(Debug)]
pub struct Index<T: Analyzer> {
    path: Option<PathBuf>,
    key: Option<String>,
    analyzer: T,
    documents: Vec<ParsedDocument>,
    /// BTreeMap<Token, BTreeMap<Count of occurances, Vec<Document Vec ID>>>
    tokens: BTreeMap<String, BTreeMap<usize, Vec<usize>>>,
}

impl<T: Analyzer> Index<T> {
    pub fn with_analyzer(mut self, analyzer: T) -> Index<T> {
        self.analyzer = analyzer;
        self
    }

    pub fn search(&self, search: Search) -> Vec<Document> {
        self.documents.iter().map(|a| a.document.clone()).collect()
    }

    pub fn add_document(&mut self, document: Document) -> Result<String, String> {
        if let Some(ref key) = self.key {
            // Delete the old doc!
        }

        let doc = self.analyzer.parse(document);
        let id = self.documents.len();
        self.documents.push(doc);
        let ref doc = self.documents.get(id).unwrap();
        for (token, count) in &doc.tokens {
            let part = self.tokens.entry(token.clone()).or_insert(BTreeMap::new());
            let ref mut entries = *part.entry(count.clone()).or_insert(vec![]);
            entries.push(id);
        }
        if let Some(ref path) = self.path {
            // Save some shit :)
        }
        Ok("Success!".to_string())
    }
}

impl Index<StandardAnalyzer> {
    pub fn new() -> Index<StandardAnalyzer> {
        Index {
            path: None,
            key: None,
            analyzer: StandardAnalyzer::new(),
            documents: vec![],
            tokens: BTreeMap::new(),
        }
    }

    pub fn persistent() -> Index<StandardAnalyzer> {
        unimplemented!()
    }
}
