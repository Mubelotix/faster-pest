// expr_pest
pub fn parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Result<&'i [u8], Error> {
    let idents_len = idents.len();
    if let Ok(input) = parse_inner_eid(input, {idents}) {
        Ok(input)
    } else {
        unsafe { idents.set_len(idents_len); }
        Ok(input)
    }
}
pub fn quick_parse_expr_id<'i, 'b>(input: &'i [u8], formatted_idents) -> Option<&'i [u8]> {
    let idents_len = idents.len();
    if let Some(input) = quick_parse_inner_eid(input, inner_idents) {
        Some(input)
    } else {
        unsafe { idents.set_len(idents_len); }
        Some(input)
    }
}
