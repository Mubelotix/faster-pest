#![feature(test)]

use std::hint::black_box;

extern crate test;

#[path = "../examples/lightgrep/main.rs"]
mod lightgrep;

use faster_pest::*;
use test::Bencher;
use lightgrep::*;

#[bench]
fn lightgrep_as_is(b: &mut Bencher) {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/lightgrep/input.txt") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/lightgrep/input.txt") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };

    b.iter(|| black_box({
        LightgrepParser::parse_file(&unparsed_file).expect("unsuccessful parse");
    }));
}

#[bench]
fn lightgrep_to_rust(b: &mut Bencher) {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/lightgrep/input.txt") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/lightgrep/input.txt") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };

    b.iter(|| black_box({
        let output = LightgrepParser::parse_file(&unparsed_file).map_err(|e| e.print(unparsed_file.as_str())).unwrap();
        let file = output.into_iter().next().unwrap();
        let main_object = file.children().next().unwrap();    
        let output = ExpressionRationnelle::from_ident_ref(main_object);
    }));
}
