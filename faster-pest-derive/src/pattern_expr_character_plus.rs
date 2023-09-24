// expr_pest
pub fn parse_expr_id<'i, 'b>(mut input: &'i [u8], formatted_idents) -> Result<&'i [u8], Error> {
    let i = input.iter().position(|c| !(character_condition)).unwrap_or(input.len());
    if i == 0 {
        return Err(Error::new(ErrorKind::Expected("character_condition"), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"));
    }
    Ok(unsafe { input.get_unchecked(i..) })
}
pub fn quick_parse_expr_id<'i, 'b>(mut input: &'i [u8], formatted_idents) -> Option<&'i [u8]> {
    let i = input.iter().position(|c| !(character_condition)).unwrap_or(input.len());
    if i == 0 {
        return None;
    }
    Some(unsafe { input.get_unchecked(i..) })
}
