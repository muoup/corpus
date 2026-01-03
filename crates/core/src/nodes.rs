use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
    sync::RwLock,
};

// --- Public Interface ---

pub trait HashNodeInner {
    fn hash(&self) -> u64;
    fn size(&self) -> u64;
}

pub struct Hashing;

#[derive(Debug)]
pub struct HashNode<T: HashNodeInner> {
    pub value: Rc<T>,
    pub hash: u64,
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
                hash,
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
            value: self.value.clone(),
            hash: self.hash,
        }
    }
}

impl<T: HashNodeInner> PartialEq for HashNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T: Display + HashNodeInner> Display for HashNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: HashNodeInner> Hash for HashNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl<T: HashNodeInner> From<T> for HashNode<T> {
    fn from(value: T) -> Self {
        let hash = value.hash();
        HashNode {
            value: Rc::new(value),
            hash,
        }
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
