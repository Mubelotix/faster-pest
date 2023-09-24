#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rule {
    array,
    boolean,
    escaped_char,
    file,
    null,
    number,
    object,
    property,
    string,
}

#[derive(Debug, Copy, Clone)]
pub enum Ident<'i> {
    Array(&'i str),
    Boolean(&'i str),
    Escaped_char(&'i str),
    File(&'i str),
    Null(&'i str),
    Number(&'i str),
    Object(&'i str),
    Property(&'i str),
    String(&'i str),
}

impl<'i> IdentTrait for Ident<'i> {
    type Rule = Rule;

    fn as_rule(&self) -> Rule {
        match self {
            Ident::Array(_) => Rule::array,
            Ident::Boolean(_) => Rule::boolean,
            Ident::Escaped_char(_) => Rule::escaped_char,
            Ident::File(_) => Rule::file,
            Ident::Null(_) => Rule::null,
            Ident::Number(_) => Rule::number,
            Ident::Object(_) => Rule::object,
            Ident::Property(_) => Rule::property,
            Ident::String(_) => Rule::string,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Ident::Array(s) => s,
            Ident::Boolean(s) => s,
            Ident::Escaped_char(s) => s,
            Ident::File(s) => s,
            Ident::Null(s) => s,
            Ident::Number(s) => s,
            Ident::Object(s) => s,
            Ident::Property(s) => s,
            Ident::String(s) => s,
        }
    }
}

#[automatically_derived]
impl JsonParser {
    pub fn parse(rule: Rule, input: &str) -> Result<Pairs2<Ident>, Error> {
        let mut idents = Vec::with_capacity(500); // TODO: refine 500
        match rule {
            Rule::array => JsonParser_faster_pest::parse_array(input.as_bytes(), &mut idents)?,
            Rule::boolean => JsonParser_faster_pest::parse_boolean(input.as_bytes(), &mut idents)?,
            Rule::escaped_char => JsonParser_faster_pest::parse_escaped_char(input.as_bytes(), &mut idents)?,
            Rule::file => JsonParser_faster_pest::parse_file(input.as_bytes(), &mut idents)?,
            Rule::null => JsonParser_faster_pest::parse_null(input.as_bytes(), &mut idents)?,
            Rule::number => JsonParser_faster_pest::parse_number(input.as_bytes(), &mut idents)?,
            Rule::object => JsonParser_faster_pest::parse_object(input.as_bytes(), &mut idents)?,
            Rule::property => JsonParser_faster_pest::parse_property(input.as_bytes(), &mut idents)?,
            Rule::string => JsonParser_faster_pest::parse_string(input.as_bytes(), &mut idents)?,
        };
        Ok(unsafe { Pairs2::from_idents(idents, input) })
    }
}

#[automatically_derived]
#[allow(clippy::all)]
mod JsonParser_faster_pest {
    use super::*;

pub fn parse_WSP<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    parse_anon_0000(input, )
}

pub fn quick_parse_WSP<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    quick_parse_anon_0000(input, )
}
impl JsonParser {
    pub fn parse_WSP(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_WSP(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_WSP(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_array<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0001(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Array(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_array<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0001(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Array(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_array(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_array(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_array(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_boolean<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0002(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Boolean(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_boolean<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0002(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Boolean(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_boolean(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_boolean(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_boolean(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_escaped_char<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0003(input, ) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Escaped_char(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_escaped_char<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0003(input, ) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Escaped_char(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_escaped_char(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_escaped_char(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_escaped_char(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_file<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0004(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::File(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_file<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0004(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::File(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_file(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_file(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_file(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_null<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0005(input, ) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Null(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_null<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0005(input, ) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Null(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_null(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_null(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_null(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_number<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0006(input, ) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Number(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_number<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0006(input, ) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Number(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_number(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_number(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_number(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_object<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0007(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Object(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_object<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0007(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Object(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_object(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_object(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_object(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_outer_string<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    parse_anon_0008(input, idents)
}

pub fn quick_parse_outer_string<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    quick_parse_anon_0008(input, idents)
}
impl JsonParser {
    pub fn parse_outer_string(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_outer_string(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_outer_string(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_property<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0009(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Property(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_property<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0009(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::Property(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_property(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_property(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_property(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_string<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_anon_0010(input, idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::String(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_string<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_anon_0010(input, idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::String(content), idents.len()); }
    Some(new_input)
}
impl JsonParser {
    pub fn parse_string(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_string(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_string(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
pub fn parse_value<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    parse_anon_0011(input, idents)
}

pub fn quick_parse_value<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    quick_parse_anon_0011(input, idents)
}
impl JsonParser {
    pub fn parse_value(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_value(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_value(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}

pub fn parse_EOI<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.is_empty() {
        Ok(input)
    } else {
        Err(Error::new(ErrorKind::Expected("EOI"), unsafe{std::str::from_utf8_unchecked(input)}, "EOI"))
    }
}
pub fn quick_parse_EOI<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.is_empty() {
        Some(input)
    } else {
        None
    }
}

 // TODO
                   pub fn parse_SOI<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
                       Ok(input)
                   }
                   pub fn quick_parse_SOI<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
                       Some(input)
                   }

                   // ((c == &32) || (c == &10) || (c == &9) || (c == &13))*
pub fn parse_anon_0000<'i, 'b>(mut input: &'i [u8], ) -> Result<&'i [u8], Error> {
    let i = input.iter().position(|c| !((c == &32) || (c == &10) || (c == &9) || (c == &13))).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return Err(Error::new(ErrorKind::Expected("(c == &32) || (c == &10) || (c == &9) || (c == &13)"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0000 ((c == &32) || (c == &10) || (c == &9) || (c == &13))*"));
    //NON-EMPTY }
    Ok(unsafe { input.get_unchecked(i..) })
}
pub fn quick_parse_anon_0000<'i, 'b>(mut input: &'i [u8], ) -> Option<&'i [u8]> {
    let i = input.iter().position(|c| !((c == &32) || (c == &10) || (c == &9) || (c == &13))).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return None;
    //NON-EMPTY }
    Some(unsafe { input.get_unchecked(i..) })
}
// (((c == &91)) ~ ((value ~ (((c == &44)) ~ value)* ~ ((c == &93))) | (WSP ~ ((c == &93)))))
pub fn parse_anon_0001<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0013(input, ).map_err(|e| e.with_trace("anon_0001-0 (((c == &91)) ~ ((value ~ (((c == &44)) ~ value)* ~ ((c == &93))) | (WSP ~ ((c == &93)))))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0020(input, idents).map_err(|e| e.with_trace("anon_0001-1 (((c == &91)) ~ ((value ~ (((c == &44)) ~ value)* ~ ((c == &93))) | (WSP ~ ((c == &93)))))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0001<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_anon_0013(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0020(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// ("true" | "false")
pub fn parse_anon_0002<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_anon_0021(input, ) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0022(input, ) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO: remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_anon_0021(input, ).unwrap_err());
    errors.push(parse_anon_0022(input, ).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0002 ("true" | "false")"#))
}
pub fn quick_parse_anon_0002<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_anon_0021(input, ) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0022(input, ) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}
// (((c == &92)) ~ ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116)))
pub fn parse_anon_0003<'i, 'b>(mut input: &'i [u8], ) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0023(input, ).map_err(|e| e.with_trace("anon_0003-0 (((c == &92)) ~ ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0024(input, ).map_err(|e| e.with_trace("anon_0003-1 (((c == &92)) ~ ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0003<'i, 'b>(mut input: &'i [u8], ) -> Option<&'i [u8]> {
    input = quick_parse_anon_0023(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0024(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// (SOI ~ value ~ EOI)
pub fn parse_anon_0004<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_SOI(input, ).map_err(|e| e.with_trace("anon_0004-0 (SOI ~ value ~ EOI)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_value(input, idents).map_err(|e| e.with_trace("anon_0004-1 (SOI ~ value ~ EOI)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_EOI(input, ).map_err(|e| e.with_trace("anon_0004-2 (SOI ~ value ~ EOI)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0004<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_SOI(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_value(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_EOI(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// "null"
pub fn parse_anon_0005<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.starts_with(b"null") {
        Ok(unsafe { input.get_unchecked("null".len()..) })
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("null"), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0005 "null""#))
    }
}
pub fn quick_parse_anon_0005<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.starts_with(b"null") {
        Some(unsafe { input.get_unchecked("null".len()..) })
    } else {
        None
    }
}
// (c.is_ascii_digit() || (c == &45) || (c == &46))*
pub fn parse_anon_0006<'i, 'b>(mut input: &'i [u8], ) -> Result<&'i [u8], Error> {
    let i = input.iter().position(|c| !(c.is_ascii_digit() || (c == &45) || (c == &46))).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return Err(Error::new(ErrorKind::Expected("c.is_ascii_digit() || (c == &45) || (c == &46)"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0006 (c.is_ascii_digit() || (c == &45) || (c == &46))*"));
    //NON-EMPTY }
    Ok(unsafe { input.get_unchecked(i..) })
}
pub fn quick_parse_anon_0006<'i, 'b>(mut input: &'i [u8], ) -> Option<&'i [u8]> {
    let i = input.iter().position(|c| !(c.is_ascii_digit() || (c == &45) || (c == &46))).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return None;
    //NON-EMPTY }
    Some(unsafe { input.get_unchecked(i..) })
}
// (((c == &123)) ~ ((property ~ (((c == &44)) ~ property)* ~ ((c == &125))) | (WSP ~ ((c == &125)))))
pub fn parse_anon_0007<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0026(input, ).map_err(|e| e.with_trace("anon_0007-0 (((c == &123)) ~ ((property ~ (((c == &44)) ~ property)* ~ ((c == &125))) | (WSP ~ ((c == &125)))))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0032(input, idents).map_err(|e| e.with_trace("anon_0007-1 (((c == &123)) ~ ((property ~ (((c == &44)) ~ property)* ~ ((c == &125))) | (WSP ~ ((c == &125)))))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0007<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_anon_0026(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0032(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// (((c == &34)) ~ string ~ ((c == &34)))
pub fn parse_anon_0008<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0033(input, ).map_err(|e| e.with_trace("anon_0008-0 (((c == &34)) ~ string ~ ((c == &34)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_string(input, idents).map_err(|e| e.with_trace("anon_0008-1 (((c == &34)) ~ string ~ ((c == &34)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0033(input, ).map_err(|e| e.with_trace("anon_0008-2 (((c == &34)) ~ string ~ ((c == &34)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0008<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_anon_0033(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_string(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0033(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)
pub fn parse_anon_0009<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0009-0 (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_outer_string(input, idents).map_err(|e| e.with_trace("anon_0009-1 (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0009-2 (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0034(input, ).map_err(|e| e.with_trace("anon_0009-3 (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_value(input, idents).map_err(|e| e.with_trace("anon_0009-4 (WSP ~ outer_string ~ WSP ~ ((c == &58)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0009<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_outer_string(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0034(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_value(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}

// (((!((c == &34) || (c == &92)) && true))+ | escaped_char)*
pub fn parse_anon_0010<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    while let Ok(new_input) = parse_anon_0037(input, idents) {
        input = new_input;
        
    }
    Ok(input)
}
pub fn quick_parse_anon_0010<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    
    while let Some(new_input) = quick_parse_anon_0037(input, idents) {
        input = new_input;
        
    }
    Some(input)
}

// (WSP ~ (outer_string | object | array | boolean | null | number) ~ WSP)
pub fn parse_anon_0011<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0011-0 (WSP ~ (outer_string | object | array | boolean | null | number) ~ WSP)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0038(input, idents).map_err(|e| e.with_trace("anon_0011-1 (WSP ~ (outer_string | object | array | boolean | null | number) ~ WSP)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0011-2 (WSP ~ (outer_string | object | array | boolean | null | number) ~ WSP)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0011<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0038(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// ((c == &32) || (c == &10) || (c == &9) || (c == &13))
pub fn parse_anon_0012<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &32) || (c == &10) || (c == &9) || (c == &13) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0012 ((c == &32) || (c == &10) || (c == &9) || (c == &13))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0012 ((c == &32) || (c == &10) || (c == &9) || (c == &13))"))
    }
}
pub fn quick_parse_anon_0012<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &32) || (c == &10) || (c == &9) || (c == &13) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((c == &91))
pub fn parse_anon_0013<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &91) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0013 ((c == &91))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0013 ((c == &91))"))
    }
}
pub fn quick_parse_anon_0013<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &91) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((c == &44))
pub fn parse_anon_0014<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &44) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0014 ((c == &44))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0014 ((c == &44))"))
    }
}
pub fn quick_parse_anon_0014<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &44) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// (((c == &44)) ~ value)
pub fn parse_anon_0015<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0014(input, ).map_err(|e| e.with_trace("anon_0015-0 (((c == &44)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_value(input, idents).map_err(|e| e.with_trace("anon_0015-1 (((c == &44)) ~ value)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0015<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_anon_0014(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_value(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}

// (((c == &44)) ~ value)*
pub fn parse_anon_0016<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    while let Ok(new_input) = parse_anon_0015(input, idents) {
        input = new_input;
        
    }
    Ok(input)
}
pub fn quick_parse_anon_0016<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    
    while let Some(new_input) = quick_parse_anon_0015(input, idents) {
        input = new_input;
        
    }
    Some(input)
}

// ((c == &93))
pub fn parse_anon_0017<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &93) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0017 ((c == &93))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0017 ((c == &93))"))
    }
}
pub fn quick_parse_anon_0017<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &93) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// (value ~ (((c == &44)) ~ value)* ~ ((c == &93)))
pub fn parse_anon_0018<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_value(input, idents).map_err(|e| e.with_trace("anon_0018-0 (value ~ (((c == &44)) ~ value)* ~ ((c == &93)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0016(input, idents).map_err(|e| e.with_trace("anon_0018-1 (value ~ (((c == &44)) ~ value)* ~ ((c == &93)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0017(input, ).map_err(|e| e.with_trace("anon_0018-2 (value ~ (((c == &44)) ~ value)* ~ ((c == &93)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0018<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_value(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0016(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0017(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// (WSP ~ ((c == &93)))
pub fn parse_anon_0019<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0019-0 (WSP ~ ((c == &93)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0017(input, ).map_err(|e| e.with_trace("anon_0019-1 (WSP ~ ((c == &93)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0019<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0017(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// ((value ~ (((c == &44)) ~ value)* ~ ((c == &93))) | (WSP ~ ((c == &93))))
pub fn parse_anon_0020<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_anon_0018(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0019(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO: remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_anon_0018(input, idents).unwrap_err());
    errors.push(parse_anon_0019(input, idents).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0020 ((value ~ (((c == &44)) ~ value)* ~ ((c == &93))) | (WSP ~ ((c == &93))))"#))
}
pub fn quick_parse_anon_0020<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_anon_0018(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0019(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}
// "true"
pub fn parse_anon_0021<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.starts_with(b"true") {
        Ok(unsafe { input.get_unchecked("true".len()..) })
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("true"), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0021 "true""#))
    }
}
pub fn quick_parse_anon_0021<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.starts_with(b"true") {
        Some(unsafe { input.get_unchecked("true".len()..) })
    } else {
        None
    }
}
// "false"
pub fn parse_anon_0022<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.starts_with(b"false") {
        Ok(unsafe { input.get_unchecked("false".len()..) })
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("false"), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0022 "false""#))
    }
}
pub fn quick_parse_anon_0022<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.starts_with(b"false") {
        Some(unsafe { input.get_unchecked("false".len()..) })
    } else {
        None
    }
}
// ((c == &92))
pub fn parse_anon_0023<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &92) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0023 ((c == &92))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0023 ((c == &92))"))
    }
}
pub fn quick_parse_anon_0023<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &92) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116))
pub fn parse_anon_0024<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0024 ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0024 ((c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116))"))
    }
}
pub fn quick_parse_anon_0024<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &34) || (c == &92) || (c == &47) || (c == &98) || (c == &102) || (c == &110) || (c == &114) || (c == &116) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// (c.is_ascii_digit() || (c == &45) || (c == &46))
pub fn parse_anon_0025<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if c.is_ascii_digit() || (c == &45) || (c == &46) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0025 (c.is_ascii_digit() || (c == &45) || (c == &46))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0025 (c.is_ascii_digit() || (c == &45) || (c == &46))"))
    }
}
pub fn quick_parse_anon_0025<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if c.is_ascii_digit() || (c == &45) || (c == &46) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((c == &123))
pub fn parse_anon_0026<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &123) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0026 ((c == &123))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0026 ((c == &123))"))
    }
}
pub fn quick_parse_anon_0026<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &123) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// (((c == &44)) ~ property)
pub fn parse_anon_0027<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_anon_0014(input, ).map_err(|e| e.with_trace("anon_0027-0 (((c == &44)) ~ property)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_property(input, idents).map_err(|e| e.with_trace("anon_0027-1 (((c == &44)) ~ property)"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0027<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_anon_0014(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_property(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}

// (((c == &44)) ~ property)*
pub fn parse_anon_0028<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    while let Ok(new_input) = parse_anon_0027(input, idents) {
        input = new_input;
        
    }
    Ok(input)
}
pub fn quick_parse_anon_0028<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    
    while let Some(new_input) = quick_parse_anon_0027(input, idents) {
        input = new_input;
        
    }
    Some(input)
}

// ((c == &125))
pub fn parse_anon_0029<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &125) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0029 ((c == &125))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0029 ((c == &125))"))
    }
}
pub fn quick_parse_anon_0029<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &125) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// (property ~ (((c == &44)) ~ property)* ~ ((c == &125)))
pub fn parse_anon_0030<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_property(input, idents).map_err(|e| e.with_trace("anon_0030-0 (property ~ (((c == &44)) ~ property)* ~ ((c == &125)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0028(input, idents).map_err(|e| e.with_trace("anon_0030-1 (property ~ (((c == &44)) ~ property)* ~ ((c == &125)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0029(input, ).map_err(|e| e.with_trace("anon_0030-2 (property ~ (((c == &44)) ~ property)* ~ ((c == &125)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0030<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_property(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0028(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0029(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// (WSP ~ ((c == &125)))
pub fn parse_anon_0031<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_WSP(input, idents).map_err(|e| e.with_trace("anon_0031-0 (WSP ~ ((c == &125)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    input = parse_anon_0029(input, ).map_err(|e| e.with_trace("anon_0031-1 (WSP ~ ((c == &125)))"))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_anon_0031<'i, 'b>(mut input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    input = quick_parse_WSP(input, idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    input = quick_parse_anon_0029(input, )?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
// ((property ~ (((c == &44)) ~ property)* ~ ((c == &125))) | (WSP ~ ((c == &125))))
pub fn parse_anon_0032<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_anon_0030(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0031(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO: remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_anon_0030(input, idents).unwrap_err());
    errors.push(parse_anon_0031(input, idents).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0032 ((property ~ (((c == &44)) ~ property)* ~ ((c == &125))) | (WSP ~ ((c == &125))))"#))
}
pub fn quick_parse_anon_0032<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_anon_0030(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_anon_0031(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}
// ((c == &34))
pub fn parse_anon_0033<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &34) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0033 ((c == &34))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0033 ((c == &34))"))
    }
}
pub fn quick_parse_anon_0033<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &34) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((c == &58))
pub fn parse_anon_0034<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &58) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0034 ((c == &58))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0034 ((c == &58))"))
    }
}
pub fn quick_parse_anon_0034<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (c == &58) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((!((c == &34) || (c == &92)) && true))
pub fn parse_anon_0035<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (!((c == &34) || (c == &92)) && true) {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0035 ((!((c == &34) || (c == &92)) && true))")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0035 ((!((c == &34) || (c == &92)) && true))"))
    }
}
pub fn quick_parse_anon_0035<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if (!((c == &34) || (c == &92)) && true) {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
// ((!((c == &34) || (c == &92)) && true))+
pub fn parse_anon_0036<'i, 'b>(mut input: &'i [u8], ) -> Result<&'i [u8], Error> {
    let i = input.iter().position(|c| !((!((c == &34) || (c == &92)) && true))).unwrap_or(input.len());
     if i == 0 {
        return Err(Error::new(ErrorKind::Expected("(!((c == &34) || (c == &92)) && true)"), unsafe{std::str::from_utf8_unchecked(input)}, "anon_0036 ((!((c == &34) || (c == &92)) && true))+"));
     }
    Ok(unsafe { input.get_unchecked(i..) })
}
pub fn quick_parse_anon_0036<'i, 'b>(mut input: &'i [u8], ) -> Option<&'i [u8]> {
    let i = input.iter().position(|c| !((!((c == &34) || (c == &92)) && true))).unwrap_or(input.len());
     if i == 0 {
        return None;
     }
    Some(unsafe { input.get_unchecked(i..) })
}
// (((!((c == &34) || (c == &92)) && true))+ | escaped_char)
pub fn parse_anon_0037<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_anon_0036(input, ) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_escaped_char(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO: remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_anon_0036(input, ).unwrap_err());
    errors.push(parse_escaped_char(input, idents).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0037 (((!((c == &34) || (c == &92)) && true))+ | escaped_char)"#))
}
pub fn quick_parse_anon_0037<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_anon_0036(input, ) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_escaped_char(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}
// (outer_string | object | array | boolean | null | number)
pub fn parse_anon_0038<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_outer_string(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_object(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_array(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_boolean(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_null(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_number(input, idents) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO: remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_outer_string(input, idents).unwrap_err());
    errors.push(parse_object(input, idents).unwrap_err());
    errors.push(parse_array(input, idents).unwrap_err());
    errors.push(parse_boolean(input, idents).unwrap_err());
    errors.push(parse_null(input, idents).unwrap_err());
    errors.push(parse_number(input, idents).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, r#"anon_0038 (outer_string | object | array | boolean | null | number)"#))
}
pub fn quick_parse_anon_0038<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_outer_string(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_object(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_array(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_boolean(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_null(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }
    if let Some(input) = quick_parse_number(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}

}
