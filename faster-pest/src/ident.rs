use std::borrow::Cow;

pub trait IdentTrait: Copy {
    type Rule: pest::RuleType;

    fn as_rule(&self) -> Self::Rule;
    fn as_str(&self) -> &str;
}

#[derive(Clone)]
pub struct IdentList<I: IdentTrait> {
    all_idents: Vec<(I, usize)>,
}

impl<I: IdentTrait> IdentList<I> {
    /// This is used by the generated parser to convert its output to an IdentList.
    /// **You should not ever need to use this.**
    /// 
    /// # Safety
    /// 
    /// The whole implementation assumes that the arguments of this function are valid.
    /// When this method is called by generated code, the input is guaranteed to be valid.
    pub unsafe fn from_idents(idents: Vec<(I, usize)>) -> Self {
        Self {
            all_idents: idents
        }
    }

    /// # Safety
    /// 
    /// The caller must ensure that `idx` is a valid index.
    pub unsafe fn get_unchecked(&self, idx: usize) -> IdentRef<'_, I> {
        IdentRef {
            ident_list: self,
            range: idx..self.all_idents.get_unchecked(idx).1,
        }
    }

    pub fn root(&self) -> IdentRef<I> {
        IdentRef {
            ident_list: self,
            range: 0..self.all_idents.len(),
        }
    }
}

impl<'i, I: IdentTrait> IntoIterator for &'i IdentList<I> {
    type Item = IdentRef<'i, I>;
    type IntoIter = IdentIter<'i, I>;

    fn into_iter(self) -> Self::IntoIter {
        IdentIter {
            ident_list: self,
            range: 0..self.all_idents.len(),
            i: 0,
        }
    }
}

impl<I: IdentTrait> std::fmt::Debug for IdentList<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.all_idents.iter().map(|(ident, _)| ident.as_str()))
            .finish()
    }
}


#[derive(Clone)]
pub struct IdentRef<'i, I: IdentTrait> {
    ident_list: &'i IdentList<I>,
    range: std::ops::Range<usize>,
}

impl<'i, I: IdentTrait> IdentRef<'i, I> {
    pub fn ident(&self) -> &'i I {
        // This is safe if the data is valid.
        // The data is valid because it originally comes from `Pairs2::from_idents`, which is only called with valid data.
        unsafe {
            &self.ident_list.all_idents.get_unchecked(self.range.start).0
        }
    }

    pub fn idx(&self) -> usize {
        self.range.start
    }

    pub fn as_str(&self) -> &'i str {
        self.ident().as_str()
    }

    pub fn as_rule(&self) -> I::Rule {
        self.ident().as_rule()
    }

    pub fn children_count(&self) -> usize {
        self.range.end - self.range.start - 1
    }

    pub fn children(&self) -> IdentIter<'i, I> {
        IdentIter {
            ident_list: self.ident_list,
            range: self.range.start + 1..self.range.end,
            i: 0,
        }
    }

    #[deprecated = "Use `children` instead"]
    pub fn inner(&self) -> IdentIter<'i, I> {
        self.children()
    }

    #[deprecated = "Use `children` instead"]
    pub fn into_inner(self) -> IdentIter<'i, I> {
        self.children()
    }
}

impl<'i, I: IdentTrait> AsRef<str> for IdentRef<'i, I> {
    fn as_ref(&self) -> &'i str {
        self.ident().as_str()
    }
}

impl<'i, I: IdentTrait> AsRef<I> for IdentRef<'i, I> {
    fn as_ref(&self) -> &'i I {
        self.ident()
    }
}

impl<'i, I: IdentTrait> std::fmt::Debug for IdentRef<'i, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.range.len() > 1 {
            f.debug_struct(&format!("{:?}", self.as_rule()))
                .field("text", &self.as_str())
                .field("children", &self.children())
                .finish()
        } else {
            f.debug_struct(&format!("{:?}", self.as_rule()))
                .field("text", &self.as_str())
                .finish()
        }
    }
}


#[derive(Clone)]
pub struct IdentIter<'i, I: IdentTrait> {
    ident_list: &'i IdentList<I>,
    range: std::ops::Range<usize>,
    i: usize,
}

impl<'i, I: IdentTrait> IdentIter<'i, I> {
    pub fn join_all(&self) -> Cow<'i, str> {
        // Try joining a reference first
        'try_ref: {
            if self.range.is_empty() {
                return Cow::Borrowed("");
            }
            let first_str = unsafe { self.ident_list.get_unchecked(self.range.start).as_str() };
            let mut end = unsafe { first_str.as_ptr().add(first_str.len()) };
            let others = IdentIter {
                ident_list: self.ident_list,
                range: self.range.clone(),
                i: 1,
            };
            for next in others {
                let next_str = next.as_str();
                if end == next_str.as_ptr() {
                    end = unsafe { next_str.as_ptr().add(next_str.len()) };
                } else {
                    break 'try_ref
                }
            }
            return Cow::Borrowed(unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(first_str.as_ptr(), end as usize - first_str.as_ptr() as usize)) });
        }

        // Fallback to a string
        let mut result = String::new();
        let all = IdentIter {
            ident_list: self.ident_list,
            range: self.range.clone(),
            i: 0,
        };
        for next in all {
            result.push_str(next.as_str());
        }
        result.into()
    }
}

impl<'i, I: IdentTrait> Iterator for IdentIter<'i, I> {
    type Item = IdentRef<'i, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.range.len() {
            return None;
        }

        let start = self.i + self.range.start;
        let end = unsafe {
            // This is safe if the data is valid.
            // The data is valid because it originally comes from `Pairs2::from_idents`, which is only called with valid data.
            self.ident_list.all_idents.get_unchecked(start).1
        };
        self.i = end - self.range.start;

        Some(IdentRef {
            ident_list: self.ident_list,
            range: start..end,
        })
    }
}

impl<'i, I: IdentTrait> std::fmt::Debug for IdentIter<'i, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries({
                let mut clone = self.clone();
                clone.i = 0;
                clone
            })
            .finish()
    }
}
