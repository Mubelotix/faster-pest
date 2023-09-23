impl StructIdent {
    pub fn parse_RuleVariant(input: &str) -> Result<IdentList<Ident>, Error> {
        let mut idents = Vec::with_capacity(500);
        if quick_parse_RuleVariant(input.as_bytes(), &mut idents).is_some() {
            return Ok(unsafe { IdentList::from_idents(idents) });
        }
        idents.clear();
        parse_RuleVariant(input.as_bytes(), &mut idents)?;
        Ok(unsafe { IdentList::from_idents(idents) })
    }
}
