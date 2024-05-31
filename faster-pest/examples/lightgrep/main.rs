use faster_pest::*;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "faster-pest/examples/lightgrep/grammar.pest"]
pub struct LightgrepParser;

#[derive(Debug)]
pub enum Repetition {
    ZeroOrOne,
    One,
    OneOrMore,
    Any,
}

#[derive(Debug)]
pub enum ExpressionRationnelle {
    Joker,
    Lettre(char),
    EnsembleLettre { lettres: HashSet<char>, negation: bool },
    WithRepetition(Box<ExpressionRationnelle>, Repetition),
    All(Vec<ExpressionRationnelle>),
    Any(Vec<ExpressionRationnelle>),
}

// TODO: Investigate: EOI didn't work well

impl ExpressionRationnelle {
    pub fn from_ident_ref(value: IdentRef<Ident>) -> Self {
        match value.as_rule() {
            Rule::er => {
                let mut children: Vec<_> = value.children().map(ExpressionRationnelle::from_ident_ref).collect();
                if children.len() == 1 {
                    return children.remove(0);
                }
                ExpressionRationnelle::Any(children)
            }
            Rule::erc => {
                let mut children: Vec<_> = value.children().map(ExpressionRationnelle::from_ident_ref).collect();
                if children.len() == 1 {
                    return children.remove(0);
                }
                ExpressionRationnelle::All(children)
            }
            Rule::erb => {
                let mut children = value.children();
                let ere = children.next().expect("ere");
                let ere = match ere.as_rule() {
                    Rule::er => ExpressionRationnelle::from_ident_ref(ere),
                    Rule::caractere | Rule::caractere_echape  => ExpressionRationnelle::Lettre(ere.as_str().chars().next().expect("caractere")),
                    Rule::joker => ExpressionRationnelle::Joker,
                    Rule::ens_lettre => ExpressionRationnelle::from_ident_ref(ere),
                    any => panic!("Unknown rule in ere: {:?}", any),
                };
                if let Some(rep) = children.next() {
                    let rep = match rep.as_str() {
                        "?" => Repetition::ZeroOrOne,
                        "+" => Repetition::OneOrMore,
                        "*" => Repetition::Any,
                        any => panic!("Unknown repetition: {:?}", any),
                    };
                    ExpressionRationnelle::WithRepetition(Box::new(ere), rep)
                } else {
                    ere
                }
            }
            Rule::ens_lettre => {
                let negation = value.as_str().as_bytes().get(1) == Some(&b'^');
                let mut lettres = HashSet::new();
                for element_ens_lettre in value.children() {
                    debug_assert!(element_ens_lettre.as_rule() == Rule::element_ens_lettre);
                    let mut children = element_ens_lettre.children();
                    let first = children.next().expect("first");
                    match children.next() {
                        Some(second) => {
                            let first = first.as_str().chars().next().expect("first");
                            let second = second.as_str().chars().next().expect("second");
                            for c in first..=second {
                                lettres.insert(c);
                            }
                        }
                        None => {
                            let c = first.as_str().chars().next().expect("c");
                            lettres.insert(c);
                        }
                    }
                }
                ExpressionRationnelle::EnsembleLettre { lettres, negation }
            }
            any => panic!("Unknown rule: {:?}", any),
        }
    }
}

fn main() {
    let unparsed_file = match std::fs::read_to_string("faster-pest/examples/lightgrep/input.txt") {
        Ok(s) => s,
        Err(_) => match std::fs::read_to_string("examples/lightgrep/input.txt") {
            Ok(s) => s,
            Err(e) => panic!("cannot read file: {}", e)
        }
    };

    let output = LightgrepParser::parse_file(&unparsed_file).map_err(|e| e.print(unparsed_file.as_str())).expect("unsuccessful parse");
    let file = output.into_iter().next().expect("couldn't find file rule");
    let main_object = file.children().next().expect("couldn't find main object");
    println!("{:#?}", main_object);

    let output = ExpressionRationnelle::from_ident_ref(main_object);
    println!("{:#?}", output);
}
