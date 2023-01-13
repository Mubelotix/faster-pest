#![feature(test)]

use std::{borrow::Cow, collections::BTreeMap};

extern crate test;

enum Value<'i> {
    String(Cow<'i, str>),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value<'i>>),
    Object(BTreeMap<Cow<'i, str>, Value<'i>>),
    Null,
}

fn unescape_str(s: &str) -> Cow<str> {
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
                Rule::string => Value::String(unescape(value)),
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
                        let name = unescape(name);
                        let value = property_children.next().unwrap();
                        object.insert(name, Value::from_ident_ref(value));
                    }
                    Value::Object(object)
                }
                Rule::null => Value::Null,
                Rule::property | Rule::file | Rule::escaped_char => unreachable!(),
            }
        }
    }
    
    fn unescape<'i>(s: IdentRef<'i, Ident>) -> Cow<'i, str> {
        let children_count = s.children_count();
        if children_count == 0 {
            return Cow::Borrowed(s.as_str());
        }
        let mut unescaped = String::with_capacity(s.as_str().len() - children_count);
        let mut i = 0;
        let start_addr = s.as_str().as_ptr() as usize;
        for escaped_char in s.children() {
            let end = escaped_char.as_str().as_ptr() as usize - start_addr;
            unescaped.push_str(s.as_str().get(i..end).unwrap());
            match unsafe { escaped_char.as_str().as_bytes().get_unchecked(1) } {
                b'"' => unescaped.push('"'),
                b'\\' => unescaped.push('\\'),
                b'/' => unescaped.push('/'),
                b'b' => unescaped.push('\x08'),
                b'f' => unescaped.push('\x0c'),
                b'n' => unescaped.push('\n'),
                b'r' => unescaped.push('\r'),
                b't' => unescaped.push('\t'),
                b'u' => {
                    // Warning when you implement this, you might want to increase the capacity of the string set above
                    unimplemented!()
                }
                _ => unreachable!()
            }
            i = end + 2;
        }
        Cow::Owned(unescaped)
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
