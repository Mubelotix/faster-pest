#![feature(test)]

use std::{borrow::Cow, collections::{HashMap, BTreeMap}};

extern crate test;

enum Value<'i> {
    String(Cow<'i, str>),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value<'i>>),
    Object(BTreeMap<Cow<'i, str>, Value<'i>>),
    Null,
}

fn json_text_to_string(s: &str) -> Cow<str> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if bytes[i] == b'\\' {
            let mut result = bytes.to_vec();
            let mut j = i;
            while j < result.len() {
                if result[j] == b'\\' {
                    result.remove(j);
                    match result[j] {
                        b'n' => result[j] = b'\n',
                        b'\\'  | b'"' | b'/' => (),
                        b't' => result[j] = b'\t',
                        b'r' => result[j] = b'\r',
                        b'b' => result[j] = b'\x08',
                        b'f' => result[j] = b'\x0C',
                        _ => todo!()
                    }
                }
                j += 1;
            }
            return Cow::Owned(unsafe { String::from_utf8_unchecked(result) })
        }
        i += 1;
    }
    Cow::Borrowed(s)
}

mod pest_classic_json {
    use std::hint::black_box;

    use pest_derive::Parser;
    use pest::Parser;
    use test::Bencher;

    #[derive(Parser)]
    #[grammar = "../faster-pest/examples/json/grammar.pest"]
    pub struct JsonParser {
    
    }

    #[bench]
    fn json_as_is(b: &mut Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/json/input.json") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/json/input.json") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            JsonParser::parse(Rule::file, &unparsed_file)
        }));
    }
}

mod faster_pest_json {
    use std::hint::black_box;
    use super::*;
    use faster_pest::*;
    use test::Bencher;

    impl<'i> Value<'i> {
        fn from_ident_ref(value: IdentRef<'i, Ident>) -> Self {
            match value.as_rule() {
                Rule::string => Value::String(json_text_to_string(value.as_str())),
                Rule::number => Value::Number(value.as_str().parse().unwrap()),
                Rule::boolean => Value::Boolean(value.as_str() == "true"),
                Rule::array => {
                    let mut array = Vec::new();
                    array.extend(value.children().map(Value::from_ident_ref));
                    Value::Array(array)
                }
                Rule::object => {
                    let mut object = BTreeMap::new();
                    for property in value.children() {
                        let mut property_children = property.children();
                        let name = property_children.next().unwrap();
                        let name = json_text_to_string(name.as_str());
                        let value = property_children.next().unwrap();
                        object.insert(name, Value::from_ident_ref(value));
                    }
                    Value::Object(object)
                }
                Rule::null => Value::Null,
                Rule::property | Rule::file => unreachable!(),
            }
        }
    }    

    #[derive(Parser)]
    #[grammar = "faster-pest/examples/json/grammar.pest"]
    pub struct JsonParser {
    
    }

    #[bench]
    fn json_as_is(b: &mut Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/json/input.json") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/json/input.json") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            JsonParser::parse_file(&unparsed_file).expect("unsuccessful parse");
        }));
    }

    #[bench]
    fn json_to_rust(b: &mut Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/json/input.json") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/json/input.json") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            let output = JsonParser::parse_file(&unparsed_file).map_err(|e| e.print(unparsed_file.as_str())).unwrap();
            let file = output.into_iter().next().unwrap();
            let main_object = file.children().next().unwrap();
            let output = Value::from_ident_ref(main_object);
        }));
    }
}

mod serde {
    use std::hint::black_box;

    #[bench]
    fn json_to_rust(b: &mut test::Bencher) {
        let unparsed_file = match std::fs::read_to_string("faster-pest/examples/json/input.json") {
            Ok(s) => s,
            Err(_) => match std::fs::read_to_string("examples/json/input.json") {
                Ok(s) => s,
                Err(e) => panic!("cannot read file: {}", e)
            }
        };

        b.iter(|| black_box({
            serde_json::from_str::<serde_json::Value>(&unparsed_file).unwrap()
        }));
    }
}
