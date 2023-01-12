use crate::*;
use std::collections::HashMap;

pub struct IdRegistry {
    ids: HashMap<String, usize>,
    next: usize,
}

impl IdRegistry {
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
            next: 0,
        }
    }

    pub fn id(&mut self, expr: &FPestExpr) -> String {
        match expr {
            FPestExpr::Ident(ident) => ident.to_string(),
            expr => {
                let id = format!("{:?}", expr);
                let id = self.ids.entry(id).or_insert_with(|| {
                    let id = self.next;
                    self.next += 1;
                    id
                });
                format!("anon_{id:0>4}")
            }
        }
    }
}
