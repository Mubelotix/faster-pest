use faster_pest::*;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "faster-pest/examples/po/grammar.pest"]
pub struct POParser;

fn main() {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/po/input.po") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/po/input.po") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };

    let file = POParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::entry => {
                

                println!("{:#?}", line.inner());
            }
            //Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
