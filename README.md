kontex [![](https://travis-ci.org/ChrisMacNaughton/kontex.svg?branch=master)](https://travis-ci.org/ChrisMacNaughton/kontex)[![](https://ci.appveyor.com/api/projects/status/rvplgkaeumlfk397?svg=true)](https://ci.appveyor.com/project/ChrisMacNaughton/kontex) [![](https://img.shields.io/crates/v/kontex.svg)](https://crates.io/crates/kontex) [![](https://coveralls.io/repos/github/ChrisMacNaughton/kontex/badge.svg?branch=master)](https://coveralls.io/github/ChrisMacNaughton/kontex?branch=master)
===========

Kontex is a full text search index.

## Examples

```rust
use kontex::{Document, Index, Search};
use std::collections::HashMap;

let mut index = Index::new();

let mut attrs = HashMap::new();
attrs.insert("text", "Hello, World!");
attrs.insert("title", "Helo");
let document = Document::from_attributes(attrs);
let _ = index.add_document(document.clone());

let mut search: HashMap<&str, &str> = HashMap::new();
search.insert("*", "world");
let result = index.search(Search::from_attributes(search));

println!("{:?}", index);
assert_eq!(*result.first().unwrap(), document);
```