use std::collections::HashMap;
//use fxhash::FxBuildHasher;

//Because we know that our keys will be the char type, which takes up 4 bytes,
//we can use a hash function that's more suited for short keys. 
// (By the way, in case you were wondering, it probably still makes sense to use a hash function for char values, 
// even though we could just use the bits' values directly as they're all a fixed sized
//type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: HashMap<char, Node>,
    //children: FxHashMap<char, TrieNode>,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        if !node.at_end {
            node.at_end = true;
            self.len += 1;
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.at_end
    }
}

