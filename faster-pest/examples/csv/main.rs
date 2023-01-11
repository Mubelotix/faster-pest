use faster_pest::*;

#[derive(Parser)]
#[grammar = "faster-pest/examples/csv/grammar.pest"]
struct CSVParser {

}

fn main() {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/csv/input.csv") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/csv/input.csv") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };
    
    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;s

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            // TODO Rule::EOI => (),
            o => println!("Unexpected {o:?}")
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
