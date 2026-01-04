use crate::base::nodes::{HashNode, HashNodeInner};
use std::collections::HashMap;

pub struct Substitution<T: HashNodeInner> {
    bindings: HashMap<u32, HashNode<T>>,
}

impl<T: HashNodeInner> Substitution<T> {
    pub fn new() -> Self {
        Substitution {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, index: u32, term: HashNode<T>) {
        self.bindings.insert(index, term);
    }

    pub fn get(&self, index: u32) -> Option<&HashNode<T>> {
        self.bindings.get(&index)
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn contains(&self, index: u32) -> bool {
        self.bindings.contains_key(&index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &HashNode<T>)> {
        self.bindings.iter()
    }

    pub fn compose(&self, other: &Substitution<T>) -> Substitution<T> {
        let mut result = self.clone();
        for (idx, term) in other.iter() {
            result.bind(*idx, term.clone());
        }
        result
    }

    pub fn apply_to_var(&self, var_idx: u32) -> Option<&HashNode<T>> {
        self.get(var_idx)
    }
}

impl<T: HashNodeInner> Clone for Substitution<T> {
    fn clone(&self) -> Self {
        Substitution {
            bindings: self.bindings.clone(),
        }
    }
}

impl<T: HashNodeInner> std::fmt::Debug for Substitution<T>
    where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Substitution({:?})", self.bindings)
    }
}

impl<T: HashNodeInner> Default for Substitution<T> {
    fn default() -> Self {
        Self::new()
    }
}
