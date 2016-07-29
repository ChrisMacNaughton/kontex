use super::document::{Document, ParsedDocument};

pub trait Analyzer {
    fn new() -> Self;
    fn parse(&self, Document) -> ParsedDocument;
}
