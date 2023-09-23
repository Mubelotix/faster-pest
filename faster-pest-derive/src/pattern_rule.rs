pub fn parse_RuleVariant<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match parse_top_expr_id(input, formatted_idents) {
        Ok(input) => input,
        Err(e) => {
            unsafe { idents.set_len(idents_len); }
            return Err(e);
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::IdentVariant(content), idents.len()); }
    Ok(new_input)
}

pub fn quick_parse_RuleVariant<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if idents_len == idents.capacity() {
        idents.reserve(500);
    }
    unsafe { idents.set_len(idents_len + 1); }
    let new_input = match quick_parse_top_expr_id(input, formatted_idents) {
        Some(input) => input,
        None => {
            unsafe { idents.set_len(idents_len); }
            return None;
        }
    };
    let content = unsafe { std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) };
    unsafe { *idents.get_unchecked_mut(idents_len) = (Ident::IdentVariant(content), idents.len()); }
    Some(new_input)
}
