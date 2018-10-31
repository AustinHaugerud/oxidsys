use pest::Parser;

#[derive(Parser)]
#[grammar = "language/oxid.pest"]
pub struct OxidParser;
