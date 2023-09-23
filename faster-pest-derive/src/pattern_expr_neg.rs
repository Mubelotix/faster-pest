// expr_pest
pub fn parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if parse_inner_id(input, inner_idents).is_err() {
        unsafe { idents.set_len(idents_len); }
        Ok(input)
    } else {
        Err(Error::new(ErrorKind::NegPredFailed("inner_id"), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"))
    }
}
pub fn quick_parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if quick_parse_inner_id(input, inner_idents).is_none() {
        unsafe { idents.set_len(idents_len); } // TODO: remove this
        Some(input)
    } else {
        None
    }
}
