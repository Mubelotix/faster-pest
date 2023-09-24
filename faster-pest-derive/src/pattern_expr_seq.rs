// expr_pest
pub fn parse_expr_id<'i, 'b>(
    mut input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Result<&'i [u8], Error> {
    
    // TODO note

    input = parse_seq_item_id(input, seq_idents).map_err(|e| e.with_trace(r#"expr_id-seq_n expr_pest"#))?; //WSP while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}
pub fn quick_parse_expr_id<'i, 'b>(
    mut input: &'i [u8],
    //SIG-IDENTS idents: &'b mut Vec<(Ident<'i>, usize)>
) -> Option<&'i [u8]> {
    input = quick_parse_seq_item_id(input, seq_idents)?; //WSP while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    
    Some(input)
}
