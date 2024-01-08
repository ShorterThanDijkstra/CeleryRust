use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "src/frontend/celery.pest"]
pub struct CeleryParser;

impl CeleryParser {
    pub fn from_file(filepath: &str) {
        // let _content = fs::read_to_string(filepath).unwrap();
        let parse = CeleryParser::parse(Rule::compound_type, "Int -> Bool").unwrap();
        println!("{:?}", parse);
    }
}
