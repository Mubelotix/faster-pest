// expr_pest
pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if character_condition {
            Ok(unsafe { input.get_unchecked(1..) })
        } else {
            Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest")) // TODO: remove unknown
        }
    } else {
        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"))
    }
}
pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if !input.is_empty() {
        let c = unsafe { input.get_unchecked(0) };
        if character_condition {
            Some(unsafe { input.get_unchecked(1..) })
        } else {
            None
        }
    } else {
        None
    }
}
