// expr_pest
pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.starts_with(bexpr_str) {
        Ok(unsafe { input.get_unchecked(expr_str.len()..) })
    } else {
        Err(Error::new(ErrorKind::ExpectedValue(expr_str), unsafe{std::str::from_utf8_unchecked(input)}, r#"expr_id expr_pest"#))
    }
}
pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.starts_with(bexpr_str) {
        Some(unsafe { input.get_unchecked(expr_str.len()..) })
    } else {
        None
    }
}
