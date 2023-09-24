// expr_pest
pub fn parse_expr_id<'i, 'b>(mut input: &'i [u8], formatted_idents) -> Result<&'i [u8], Error> {
    //NON-EMPTY input = parse_inner_eid(input, inner_idents)?;
    while let Ok(new_input) = parse_inner_eid(input, inner_idents) {
        input = new_input;
        //WSP //while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    }
    Ok(input)
}
pub fn quick_parse_expr_id<'i, 'b>(mut input: &'i [u8], formatted_idents) -> Option<&'i [u8]> {
    //NON-EMPTY input = quick_parse_inner_eid(input, inner_idents)?;
    while let Some(new_input) = quick_parse_inner_eid(input, inner_idents) {
        input = new_input;
        //WSP //while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    }
    Some(input)
}
