use std::collections::HashMap;

struct TrieNode{
    children: HashMap<char,TrieNode>,
    is_end: bool,
}

struct Trie {
    root: TrieNode,
}



 impl TrieNode{
    fn new() -> Self{
        Self{
        children : HashMap::new(),
        is_end: false,
        }
    }
 }
impl Trie {

    fn new() -> Self {
        Self{
            root: TrieNode::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
         let mut node = &mut self.root;
         for ch in word.chars(){
           node = node.children.entry(ch).or_insert(TrieNode::new());
        
         }
         node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        
        node.is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;

        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        
       return true
    }
}

