use super::document::{Document, ParsedDocument};

pub trait Analyzer {
    fn new() -> Self;
    fn parse(&self, Document) -> ParsedDocument;
    fn tokenize(&String) -> Vec<String>;
    fn tokenize_string(&self, &String) -> Vec<String>;
}
