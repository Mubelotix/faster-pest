// expr_pest
pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {
    if input.len() < expr_len_str {
        return Err(Error::new(ErrorKind::ExpectedValue(expr_str), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"));
    }
    for i in 0..expr_len_str {
        if input[i] != bexpr_str[i] && input[i] != bexpr_inv_str[i] {
            return Err(Error::new(ErrorKind::ExpectedValue(expr_str), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"));
        }
    }
    Ok(unsafe { input.get_unchecked(expr_len_str..) })
}
pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {
    if input.len() < expr_len_str {
        return None;
    }
    for i in 0..expr_len_str {
        if input[i] != bexpr_str[i] && input[i] != bexpr_inv_str[i] {
            return None;
        }
    }
    Some(unsafe { input.get_unchecked(expr_len_str..) })
}
