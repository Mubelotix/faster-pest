use std::rc::Rc;

use pest::*;

pub trait IdentTrait: Copy {
    type Rule: Copy;

    fn as_rule(&self) -> Self::Rule;
    fn as_str(&self) -> &str;
}


pub struct Tokens2 {

}


#[derive(Debug, Clone)]
pub struct Pair2<'i, I: IdentTrait> {
    all_idents: Rc<Vec<I>>,
    ident_range: std::ops::Range<usize>,
    initial_text: &'i str,
}

impl<'i, I: IdentTrait> Pair2<'i, I> {
    pub(crate) fn ident(&self) -> &I {
        self.all_idents.get(self.ident_range.start).unwrap()
    }

    pub fn as_rule(&self) -> I::Rule {
        self.ident().as_rule()
    }

    pub fn as_str(&self) -> &'i str {
        let start = self.ident().as_str().as_ptr() as usize - self.initial_text.as_ptr() as usize;
        let end = start + self.ident().as_str().len();
        &self.initial_text[start..end]
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
            all_idents: Rc::clone(&self.all_idents),
            ident_range: self.ident_range.start + 1..self.ident_range.end,
            initial_text: self.initial_text,
            i: 0,
        }
    }

    pub fn tokens(self) -> Tokens2 {
        unimplemented!()
    }
}


#[derive(Debug, Clone)]
pub struct Pairs2<'i, I: IdentTrait> {
    all_idents: Rc<Vec<I>>,
    ident_range: std::ops::Range<usize>,
    initial_text: &'i str,
    i: usize,
}

impl<'i, I: IdentTrait> Pairs2<'i, I> {
    pub fn from_idents(idents: Vec<I>, initial_text: &'i str) -> Self {
        Self {
            ident_range: 0..idents.len(),
            all_idents: Rc::new(idents),
            initial_text,
            i: 0,
        }
    }
}

impl<'i, I: IdentTrait + 'i> Iterator for Pairs2<'i, I> {
    type Item = Pair2<'i, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.i + self.ident_range.start) >= self.all_idents.len() || self.i >= self.ident_range.end {
            return None;
        }
        let i = self.i + self.ident_range.start;

        let inner_end = self.all_idents[i].as_str().as_ptr() as usize + self.all_idents[i].as_str().len() - self.initial_text.as_ptr() as usize;
        let start = i;
        let mut end = i + 1;
        while let Some(ident) = self.all_idents.get(end) {
            let ident_start = ident.as_str().as_ptr() as usize - self.initial_text.as_ptr() as usize;
            if ident_start >= inner_end {
                break;
            }
            end += 1;
        }

        self.i = end - self.ident_range.start;

        Some(Pair2 {
            all_idents: Rc::clone(&self.all_idents),
            initial_text: self.initial_text,
            ident_range: start..end,
        })
    }
}
