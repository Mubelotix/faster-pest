// expr_pest
pub fn parse_expr_id<'i, 'b>(
    input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Result<&'i [u8], Error> {
    //SIG-IDENTS let idents_len = idents.len();
    if parse_inner_id(
        input,
        //SIG-IDENTS idents
    ).is_err() {
        //SIG-IDENTS unsafe { idents.set_len(idents_len); }
        Ok(input)
    } else {
        Err(Error::new(ErrorKind::NegPredFailed("inner_id"), unsafe{std::str::from_utf8_unchecked(input)}, r#"expr_id expr_pest"#))
    }
}
pub fn quick_parse_expr_id<'i, 'b>(
    input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Option<&'i [u8]> {
    //SIG-IDENTS let idents_len = idents.len();
    if quick_parse_inner_id(
        input,
        //SIG-IDENTS idents
    ).is_none() {
        //SIG-IDENTS unsafe { idents.set_len(idents_len); } // TODO: remove this
        Some(input)
    } else {
        None
    }
}
