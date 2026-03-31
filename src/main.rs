mod skiplist;
mod trie;

fn main() {
    let mut trie = trie::Trie::new();
    trie.insert("hello");
    trie.insert("hell");
    println!("{:?}", trie);
}
