use corpus_core::nodes::{HashNode, HashNodeInner, NodeStorage};
use std::collections::HashMap;

pub struct Substitution {
    bindings: HashMap<u32, HashNode<u32>>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, index: u32, term: HashNode<u32>) {
        self.bindings.insert(index, term);
    }

    pub fn get(&self, index: u32) -> Option<&HashNode<u32>> {
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

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &HashNode<u32>)> {
        self.bindings.iter()
    }

    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = self.clone();
        for (idx, term) in other.iter() {
            result.bind(*idx, term.clone());
        }
        result
    }

    pub fn apply_to_var<T: HashNodeInner>(
        &self,
        var_idx: u32,
        _store: &NodeStorage<T>,
    ) -> Option<HashNode<u32>> {
        self.get(var_idx).cloned()
    }
}

impl Clone for Substitution {
    fn clone(&self) -> Self {
        Substitution {
            bindings: self.bindings.clone(),
        }
    }
}

impl std::fmt::Debug for Substitution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Substitution({:?})", self.bindings)
    }
}

impl Default for Substitution {
    fn default() -> Self {
        Self::new()
    }
}
