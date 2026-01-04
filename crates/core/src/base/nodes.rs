use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
    sync::RwLock,
};

// --- Public Interface ---

pub trait HashNodeInner: Sized {
    fn hash(&self) -> u64;
    fn size(&self) -> u64;

    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)> {
        None
    }

    /// Try to rewrite any subterm (including this node) using the given rewrite function.
    ///
    /// This is a default implementation that only tries to rewrite the top-level node.
    /// Domains can override this to recursively try subterms (e.g., for PA expressions).
    fn rewrite_any_subterm<F>(
        &self,
        node: &HashNode<Self>,
        _store: &NodeStorage<Self>,
        try_rewrite: &F,
    ) -> Option<HashNode<Self>>
    where
        F: Fn(&HashNode<Self>) -> Option<HashNode<Self>>,
    {
        // Default: just try rewriting the top-level node
        try_rewrite(node)
    }
}

pub struct Hashing;

#[derive(Debug)]
pub struct HashNode<T: HashNodeInner> {
    pub value: Rc<T>,
}

pub struct NodeStorage<T: HashNodeInner> {
    nodes: RwLock<HashMap<u64, HashNode<T>, std::hash::BuildHasherDefault<IdentityHasher>>>,
}

impl<T: HashNodeInner> NodeStorage<T> {
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::default()),
        }
    }

    pub fn get_or_insert(&self, value: T) -> HashNode<T> {
        let hash = value.hash();
        let mut nodes = self.nodes.write().unwrap();

        if let Some(existing) = nodes.get(&hash) {
            existing.clone()
        } else {
            let node = HashNode {
                value: Rc::new(value),
            };
            nodes.insert(hash, node.clone());
            node
        }
    }

    pub fn get(&self, hash: u64) -> Option<HashNode<T>> {
        let nodes = self.nodes.read().unwrap();
        nodes.get(&hash).cloned()
    }

    pub fn len(&self) -> usize {
        let nodes = self.nodes.read().unwrap();
        nodes.len()
    }

    pub fn clear(&self) {
        let mut nodes = self.nodes.write().unwrap();
        nodes.clear();
    }
}

impl<T: HashNodeInner> HashNode<T> {
    pub fn size(&self) -> u64 {
        self.value.size()
    }
    
    pub fn hash(&self) -> u64 {
        self.value.hash()
    }
}

impl Hashing {
    pub fn hash_combine(hash1: u64, hash2: u64) -> u64 {
        const MAGIC: u64 = 0x9e3779b9;

        hash1
            ^ (hash1
                .wrapping_add(MAGIC)
                .wrapping_add(hash2 << 6)
                .wrapping_add(hash2 >> 2))
    }

    pub fn root_hash(root_opcode: u8, children: &[u64]) -> u64 {
        let mut result = root_opcode as u64;
        for &h in children {
            result = Self::hash_combine(result, h);
        }
        result
    }

    pub fn opcode(name: &str) -> u8 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        (hasher.finish() % 255) as u8
    }
}

// --- Implementations ---

#[derive(Default)]
struct IdentityHasher {
    hash: u64,
}

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }
}

impl<T: HashNodeInner + Clone> Default for NodeStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HashNodeInner> Clone for HashNode<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone()
        }
    }
}

impl<T: HashNodeInner> PartialEq for HashNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.hash() == other.value.hash()
    }
}

impl<T: Display + HashNodeInner> Display for HashNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: HashNodeInner> Hash for HashNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.value.hash());
    }
}

impl<T: HashNodeInner> HashNode<T> {
    pub fn from_store(value: T, store: &NodeStorage<T>) -> Self {
        store.get_or_insert(value)
    }
}

impl HashNodeInner for u64 {
    fn hash(&self) -> u64 {
        *self
    }

    fn size(&self) -> u64 {
        1
    }
}

impl HashNodeInner for u32 {
    fn hash(&self) -> u64 {
        *self as u64
    }

    fn size(&self) -> u64 {
        1
    }
}
