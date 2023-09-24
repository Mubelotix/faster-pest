#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rule {
    RuleVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum Ident<'i> {
    IdentVariant(&'i str),
}

impl<'i> IdentTrait for Ident<'i> {
    type Rule = Rule;

    fn as_rule(&self) -> Rule {
        match self {
            Ident::IdentVariant(_) => Rule::RuleVariant,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Ident::IdentVariant(s) => s,
        }
    }
}

#[automatically_derived]
impl StructIdent {
    pub fn parse(rule: Rule, input: &str) -> Result<Pairs2<Ident>, Error> {
        let mut idents = Vec::with_capacity(500); // TODO: refine 500
        match rule {
            Rule::RuleVariant => StructIdent_faster_pest::parse_RuleVariant(input.as_bytes(), &mut idents)?,
        };
        Ok(unsafe { Pairs2::from_idents(idents, input) })
    }
}

#[automatically_derived]
#[allow(clippy::all)]
mod StructIdent_faster_pest {
    use super::*;

    // inner code
}
