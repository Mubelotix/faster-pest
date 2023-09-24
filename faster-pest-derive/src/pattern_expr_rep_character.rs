// expr_pest
pub fn parse_expr_id<'i, 'b>(
    mut input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Result<&'i [u8], Error> {
    let i = input.iter().position(|c| !(character_condition)).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return Err(Error::new(ErrorKind::Expected("character_condition"), unsafe{std::str::from_utf8_unchecked(input)}, "expr_id expr_pest"));
    //NON-EMPTY }
    Ok(unsafe { input.get_unchecked(i..) })
}
pub fn quick_parse_expr_id<'i, 'b>(
    mut input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Option<&'i [u8]> {    let i = input.iter().position(|c| !(character_condition)).unwrap_or(input.len());
    //NON-EMPTY if i == 0 {
    //NON-EMPTY    return None;
    //NON-EMPTY }
    Some(unsafe { input.get_unchecked(i..) })
}
