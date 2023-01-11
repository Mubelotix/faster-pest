use faster_pest::*;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "faster-pest/examples/ini/grammar.pest"]
pub struct INIParser;

fn main() {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/ini/input.ini") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/ini/input.ini") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };

    let file = INIParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner(); // { name }
                current_section_name = inner_rules.next().unwrap().as_str();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name: &str = inner_rules.next().unwrap().as_str();
                let value: &str = inner_rules.next().unwrap().as_str();

                // Insert an empty inner hash map if the outer hash map hasn't
                // seen this section name before.
                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            //Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
