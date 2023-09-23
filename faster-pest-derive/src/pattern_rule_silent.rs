pub fn parse_RuleVariant<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {
    parse_top_expr_id(input, formatted_idents)
}

pub fn quick_parse_RuleVariant<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {
    quick_parse_top_expr_id(input, formatted_idents)
}
