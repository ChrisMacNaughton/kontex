use std::collections::BTreeMap;
use std::path::PathBuf;

use super::analyzer::Analyzer;
use super::analyzers::standard_analyzer::StandardAnalyzer;
use super::document::{Document, ParsedDocument};
use super::Search;

#[cfg(test)]
mod tests {
    use super::*;
    use analyzer::Analyzer;
    use analyzers::standard_analyzer::StandardAnalyzer;
    use {Document, Search};
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
        search.insert("text", "world");
        let result = index.search(Search::from_attributes(search));
        println!("Result: {:?}", result);
        println!("Index: {:?}", index);
        assert_eq!(*result.first().unwrap(), document);
    }

    
    #[test]
    fn it_queries_wildcard() {

        let mut index = Index::new();

        let mut attrs = HashMap::new();
        attrs.insert("text", "Helo, World!");
        attrs.insert("title", "test");
        let document1 = Document::from_attributes(attrs);
        index.add_document(document1.clone());

        let mut attrs = HashMap::new();
        attrs.insert("text", "Helo, World!");
        attrs.insert("title", "Hi");
        let document = Document::from_attributes(attrs);
        index.add_document(document.clone());

        let mut search: HashMap<&str, &str> = HashMap::new();
        search.insert("*", "helo");
        let result = index.search(Search::from_attributes(search));
        println!("Result: {:?}", result);
        println!("Index: {:?}", index);
        assert_eq!(*result.first().unwrap(), document1);
    }

    #[test]
    fn it_sub_queries() {
        let mut index = Index::new();

        let mut attrs = HashMap::new();
        attrs.insert("text", "Helo, World!");
        attrs.insert("title", "test");
        let document = Document::from_attributes(attrs);
        index.add_document(document);

        let mut attrs = HashMap::new();
        attrs.insert("text", "Helo, World!");
        attrs.insert("title", "Helo");
        let document = Document::from_attributes(attrs);
        index.add_document(document.clone());

        let result = index.sub_search(&"title".to_string(), &"helo".to_string());

        assert_eq!(result.first().unwrap().1.document, document);
    }
}

#[derive(Debug)]
pub struct Index<T: Analyzer> {
    path: Option<PathBuf>,
    key: Option<String>,
    analyzer: T,
    documents: Vec<ParsedDocument>,
    /// BTreeMap<Section, BTreeMap<Token, BTreeMap<Count of occurances, Vec<Document Vec ID>>>
    tokens: BTreeMap<String, BTreeMap<String, BTreeMap<usize, Vec<usize>>>>,
}

impl<T: Analyzer> Index<T> {
    pub fn with_analyzer(mut self, analyzer: T) -> Index<T> {
        self.analyzer = analyzer;
        self
    }

    pub fn search(&self, search: Search) -> Vec<Document> {
        let mut unsorted: Vec<(usize, &ParsedDocument)> = vec![];
        for (ref section, ref query) in search.options {
            if section == "*" {
                for (section, _) in &self.tokens {
                    unsorted.append(&mut self.sub_search(section, query));
                }
            } else {
                unsorted.append(&mut self.sub_search(section, query));
            }
        }
        unsorted.sort_by(|a, b| a.0.cmp(&b.0));
        unsorted.iter().map(|a| a.1.document.clone()).collect::<Vec<Document>>()
    }

    fn sub_search(&self, section: &String, query: &String) -> Vec<(usize, &ParsedDocument)> {
        let mut docs: Vec<(usize, &ParsedDocument)> = vec![];
        let query = self.analyzer.tokenize_string(query);
        match self.tokens.get(section) {
            Some(ref s) => {
                for token in query {
                    match s.get(&token) {
                        Some(s) => {
                            for (ref count, ref doc_ids) in s {
                                for doc_id in doc_ids.iter() {
                                    docs.push((*count.clone(), self.documents.get(*doc_id).unwrap()))
                                }
                            }
                            
                        },
                        None => continue
                    }
                }
            },
            None => return docs
        }
        
        // docs.push(self.documents.first().unwrap());
        docs
    }

    pub fn add_document(&mut self, document: Document) -> Result<String, String> {
        if let Some(ref key) = self.key {
            // Delete the old doc!
        }

        let doc = self.analyzer.parse(document);
        let id = self.documents.len();
        self.documents.push(doc);
        let ref doc = self.documents.get(id).unwrap();
        for (token_type, tokens) in &doc.tokens {
            for (token, count) in tokens {
                let section = self.tokens.entry(token_type.clone()).or_insert(BTreeMap::new());
                let part = section.entry(token.clone()).or_insert(BTreeMap::new());
                let ref mut entries = *part.entry(count.clone()).or_insert(vec![]);
                entries.push(id);
            }
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
