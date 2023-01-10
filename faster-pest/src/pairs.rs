use std::rc::Rc;

use pest::*;

pub trait IdentTrait: Copy {
    type Rule: Copy;

    fn as_rule(&self) -> Self::Rule;
    fn as_str(&self) -> &str;
}


pub struct Tokens2 {

}


pub struct Pair2<'i, I: IdentTrait> {
    all_idents: Rc<Vec<I>>,
    initial_text: &'i str,
    start: usize,
    end: usize,
}

impl<'i, I: IdentTrait> Pair2<'i, I> {
    pub fn as_rule(&self) -> I::Rule {
        self.all_idents[self.start].as_rule()
    }

    pub fn as_str(&self) -> &str {
        self.all_idents[self.start].as_str()
    }

    #[deprecated = "Please use as_span instead"]
    pub fn into_span(self) -> Span<'i> {
        self.as_span()
    }

    pub fn as_span(&self) -> Span<'i> {
        let start = self.as_str().as_ptr() as usize - self.initial_text.as_ptr() as usize;
        let end = start + self.as_str().len();
        Span::new(self.initial_text, start, end).unwrap()
    }

    pub fn into_inner(self) -> Pairs2<'i, I> {
        Pairs2 {
            idents: Rc::new(self.all_idents[self.start + 1..self.end].to_vec()),
            initial_text: self.initial_text,
            i: 0,
        }
    }

    pub fn tokens(self) -> Tokens2 {
        unimplemented!()
    }
}



pub struct Pairs2<'i, I: IdentTrait> {
    idents: Rc<Vec<I>>,
    initial_text: &'i str,
    i: usize,
}

impl<'i, I: IdentTrait> Pairs2<'i, I> {
    pub fn from_idents(idents: Vec<I>, initial_text: &'i str) -> Self {
        Self {
            idents: Rc::new(idents),
            initial_text,
            i: 0,
        }
    }
}

impl<'i, I: IdentTrait + 'i> Iterator for Pairs2<'i, I> {
    type Item = Pair2<'i, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.idents.len() {
            return None;
        }

        let inner_end = self.idents[self.i].as_str().as_ptr() as usize + self.idents[self.i].as_str().len() - self.initial_text.as_ptr() as usize;
        let start = self.i;
        let mut end = self.i + 1;
        while let Some(ident) = self.idents.get(end) {
            let ident_start = ident.as_str().as_ptr() as usize - self.initial_text.as_ptr() as usize;
            if ident_start >= inner_end {
                break;
            }
            end += 1;
        }

        self.i = end;

        Some(Pair2 {
            all_idents: Rc::clone(&self.idents),
            initial_text: self.initial_text,
            start,
            end
        })
    }
}
