// expr_pest
pub fn parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    
    if let Some(input) = quick_parse_choice_item_id(input, ) { return Ok(input); } unsafe { idents.set_len(idents_len); }

    // TODO remove last set_len

    let mut errors = Vec::new();
    errors.push(parse_choice_item_id(input, choice_idents).unwrap_err());

    unsafe { idents.set_len(idents_len); }

    Err(Error::new(ErrorKind::All(errors), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"))
}
pub fn quick_parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Option<&'i [u8]> {
    let idents_len = idents.len();

    if let Some(input) = quick_parse_choice_item_id(input, idents) { return Some(input); } unsafe { idents.set_len(idents_len); }

    None
}
