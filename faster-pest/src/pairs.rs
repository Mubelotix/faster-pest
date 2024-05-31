use std::rc::Rc;
use crate::*;
use pest::*;

/// A [`Pair2`] is a reference to a `Ident` and its children. It mimics pest's [`Pair`](pest::iterators::Pair).  
/// It is created by [`Pairs2::next`].
#[derive(Clone)]
pub struct Pair2<'i, I: IdentTrait> {
    /// The original input that was parsed.
    original_input: &'i str,
    /// A reference to the output of the parsing.
    all_idents: Rc<Vec<(I, usize)>>,
    /// The range indicates where the [`Pair2`] is stored in `all_idents`.
    /// `all_idents[range.start]` is the ident of the [`Pair2`], and `all_idents[range.start + 1..range.end]` are the children.
    range: std::ops::Range<usize>,
}

impl<'i, I: IdentTrait> Pair2<'i, I> {
    pub fn ident(&self) -> &I {
        // This is safe if the data is valid.
        // The data is valid because it originally comes from `Pairs2::from_idents`, which is only called with valid data.
        unsafe {
            &self.all_idents.get_unchecked(self.range.start).0
        }
    }

    pub fn as_rule(&self) -> I::Rule {
        self.ident().as_rule()
    }

    pub fn as_str(&self) -> &'i str {
        // This is safe if the data is valid.
        // The data is valid because it originally comes from `Pairs2::from_idents`, which is only called with valid data.
        unsafe {
            let str_start = self.ident().as_str().as_ptr() as usize - self.original_input.as_ptr() as usize;
            let str_end = str_start + self.ident().as_str().len();    
            self.original_input.get_unchecked(str_start..str_end)
        }
    }

    #[deprecated = "Please use as_span instead"]
    pub fn into_span(self) -> Span<'i> {
        self.as_span()
    }

    pub fn as_span(&self) -> Span<'i> {
        let start = self.as_str().as_ptr() as usize - self.original_input.as_ptr() as usize;
        let end = start + self.as_str().len();
        Span::new(self.original_input, start, end).expect("Pair2::as_span: invalid span")
    }

    pub fn inner(&self) -> Pairs2<'i, I> {
        Pairs2 {
            all_idents: Rc::clone(&self.all_idents),
            range: self.range.start + 1..self.range.end,
            initial_text: self.original_input,
            i: 0,
        }
    }

    pub fn into_inner(self) -> Pairs2<'i, I> {
        Pairs2 {
            all_idents: self.all_idents,
            range: self.range.start + 1..self.range.end,
            initial_text: self.original_input,
            i: 0,
        }
    }
}

impl<'i, I: IdentTrait> std::fmt::Debug for Pair2<'i, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("{:?}", self.as_rule()))
            .field("text", &self.as_str())
            //.field("range", &self.range)
            .field("inner", &self.inner())
            .finish()
    }
}


/// A [`Pairs2`] is an iterator over [`Pair2`]s. It mimics pest's [`Pairs`](pest::iterators::Pairs).  
/// It is created by [`Parser::parse`].  
/// 
/// Iterating over it will only yield top-level children.
/// To iterate over all [`Pair2`]s, use [`Pair2::into_inner`] on yielded [`Pair2`]s.
#[derive(Clone)]
pub struct Pairs2<'i, I: IdentTrait> {
    all_idents: Rc<Vec<(I, usize)>>,
    range: std::ops::Range<usize>,
    initial_text: &'i str,
    i: usize,
}

impl<'i, I: IdentTrait> Pairs2<'i, I> {
    /// This is used by the generated parser to convert its output to a [`Pairs2`].
    /// **You should not ever need to use this.**
    /// 
    /// # Safety
    /// 
    /// The whole [Pairs2] and [Pair2] implementation assumes that the arguments of this function are valid.
    /// When this method is called by generated code, the input is guaranteed to be valid.
    pub unsafe fn from_idents(idents: Vec<(I, usize)>, initial_text: &'i str) -> Self {
        Self {
            range: 0..idents.len(),
            all_idents: Rc::new(idents),
            initial_text,
            i: 0,
        }
    }
}

impl<'i, I: IdentTrait + 'i> Iterator for Pairs2<'i, I> {
    type Item = Pair2<'i, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.range.len() {
            return None;
        }
        let start = self.i + self.range.start;
        let end = unsafe {
            // This is safe if the data is valid.
            // The data is valid because it originally comes from `Pairs2::from_idents`, which is only called with valid data.
            self.all_idents.get_unchecked(start).1
        };
        self.i = end - self.range.start;

        Some(Pair2 {
            all_idents: Rc::clone(&self.all_idents),
            original_input: self.initial_text,
            range: start..end,
        })
    }
}

impl<'i, I: IdentTrait> std::fmt::Debug for Pairs2<'i, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries({
            let mut clone = self.clone();
            clone.i = 0;
            clone
        }).finish()
    }
}
