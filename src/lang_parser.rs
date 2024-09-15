use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "jpp.pest"] // Your grammar file
pub struct JPPParser;
