#![feature(test)]

extern crate test;

mod pest_classic {
    use std::hint::black_box;

    use pest_derive::Parser;
    use pest::Parser;
    use test::Bencher;

    #[derive(Parser)]
    #[grammar = "../faster-pest/examples/csv/grammar.pest"]
    pub struct CSVParser {
    
    }

    #[bench]
    fn csv(b: &mut Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/csv/input.csv") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/csv/input.csv") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            let file = CSVParser::parse(Rule::file, &unparsed_file)
                .expect("unsuccessful parse")
                .next()
                .expect("couldn't find file rule");

            let mut field_sum: f64 = 0.0;
            let mut record_count: u64 = 0;

            for record in file.into_inner() {
                match record.as_rule() {
                    Rule::record => {
                        record_count += 1;

                        for field in record.into_inner() {
                            field_sum += field.as_str().parse::<f64>().expect("field should be a number")
                        }
                    }
                    // TODO Rule::EOI => (),
                    o => println!("Unexpected {o:?}")
                }
            }

            (field_sum, record_count)
        }));
    }
}

mod faster_pest {
    use std::hint::black_box;
    use faster_pest::*;
    use test::Bencher;

    #[derive(Parser)]
    #[grammar = "faster-pest/examples/csv/grammar.pest"]
    pub struct CSVParser {
    
    }

    #[bench]
    fn csv(b: &mut Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/csv/input.csv") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/csv/input.csv") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            let file = CSVParser::parse_file(&unparsed_file).expect("unsuccessful parse");
            let file = file.into_iter().next().expect("couldn't find file rule");

            let mut field_sum: f64 = 0.0;
            let mut record_count: u64 = 0;

            for record in file.children() {
                match record.as_rule() {
                    Rule::record => {
                        record_count += 1;

                        for field in record.children() {
                            field_sum += field.as_str().parse::<f64>().expect("field should be a number")
                        }
                    }
                    // TODO Rule::EOI => (),
                    o => println!("Unexpected {o:?}")
                }
            }

            (field_sum, record_count)
        }));
    }
}
