use pest::*;

pub trait IdentTrait: Copy {
    type Rule: Copy;

    fn as_rule(&self) -> Self::Rule;
    fn as_str(&self) -> &str;
}


pub struct Tokens2 {

}


pub struct Pair2<'i, I: IdentTrait> {
    father: &'i Pairs2<'i, I>,
    i: usize,
}

impl<'i, I: IdentTrait> Pair2<'i, I> {
    pub fn as_rule(&self) -> I::Rule {
        self.father.idents[self.i].as_rule()
    }

    pub fn as_str(&self) -> &str {
        self.father.idents[self.i].as_str()
    }

    #[deprecated = "Please use as_span instead"]
    pub fn into_span(self) -> Span<'i> {
        self.as_span()
    }

    pub fn as_span(&self) -> Span<'i> {
        let start = self.as_str().as_ptr() as usize - self.father.initial_text.as_ptr() as usize;
        let end = start + self.as_str().len();
        Span::new(self.father.initial_text, start, end).unwrap()
    }

    pub fn into_inner(self) -> Pairs2<'i, I> {
        let inner_end = self.as_str().as_ptr() as usize + self.as_str().len() - self.father.initial_text.as_ptr() as usize;

        let mut i = self.i;
        while let Some(ident) = self.father.idents.get(i) {
            let ident_start = ident.as_str().as_ptr() as usize - self.father.initial_text.as_ptr() as usize;
            if ident_start >= inner_end {
                break;
            }
            i += 1;
        }
        
        Pairs2 {
            idents: self.father.idents[self.i + 1..i].to_vec(),
            initial_text: self.father.initial_text,
            i: 0,
        }
    }

    pub fn tokens(self) -> Tokens2 {
        unimplemented!()
    }
}



pub struct Pairs2<'i, I: IdentTrait> {
    idents: Vec<I>,
    initial_text: &'i str,
    i: usize,
}

impl<'i, I: IdentTrait> Pairs2<'i, I> {
    pub fn from_idents(idents: Vec<I>, initial_text: &'i str) -> Self {
        Self {
            idents,
            initial_text,
            i: 0,
        }
    }
}

impl<'i, I: IdentTrait + 'i> Pairs2<'i, I> {
    pub fn next(&'i mut self) -> Option<Pair2<'i, I>> {
        if self.i < self.idents.len() {
            self.i += 1;
            Some(Pair2 {
                i: self.i - 1,
                father: self,
            })
        } else {
            None
        }
    }
}
