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

    let output = POParser::parse_file(&unparsed_file).map_err(|e| e.print(unparsed_file.as_str())).unwrap();
    let file = output.into_iter().next().unwrap(); // get and unwrap the `file` rule; never fails

    for line in file.children() {
        match line.as_rule() {
            Rule::entry => {
                

                println!("{:#?}", line.children());
            }
            //Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
