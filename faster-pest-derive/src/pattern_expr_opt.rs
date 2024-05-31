// expr_pest
pub fn parse_expr_id<'i, 'b>(
    input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Result<&'i [u8], Error> {
    //SIG-IDENTS let idents_len = idents.len();
    if let Ok(input) = parse_inner_eid(
        input,
        //SIG-IDENTS idents
    ) {
        Ok(input)
    } else {
        //SIG-IDENTS unsafe { idents.set_len(idents_len); }
        Ok(input)
    }
}
pub fn quick_parse_expr_id<'i, 'b>(
    input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Option<&'i [u8]> {
    //SIG-IDENTS let idents_len = idents.len();
    if let Some(input) = quick_parse_inner_eid(
        input,
        //SIG-IDENTS idents
    ) {
        Some(input)
    } else {
        //SIG-IDENTS unsafe { idents.set_len(idents_len); }
        Some(input)
    }
}
