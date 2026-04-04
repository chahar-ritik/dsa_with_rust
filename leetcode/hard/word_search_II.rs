use std::collections::HashMap;
#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>> ;26],
    word: Option<String>,
    children_count: usize,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {

        let mut trie = Trie::new();

        // Build Trie
        for word in words{
      Self::insert(&mut trie.root , word);
        }
        let mut res = vec![];
        let rows = board.len();
        let cols = board[0].len();

        for r in 0..rows {
            for c in 0..cols {
                Self::dfs(&mut board, &mut trie.root, &mut res, r, c);
            }
        }

        res
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        node: &mut TrieNode,
        res: &mut Vec<String>,
        r: usize,
        c: usize,
    ) {
        let ch  = board[r][c];
         let idx = (ch as u8 - b'a') as usize;
        
        if node.children[idx].is_none(){
            return;
        }

      
        let  next_node = node.children[idx].as_mut().unwrap();

      
        if let Some(word) = &next_node.word {
            res.push(word.clone());
            next_node.word = None; // avoid duplicates
        }


      
        board[r][c] = '#';

        let directions = [(1,0), (-1,0), (0,1), (0,-1)];

        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nc >= 0 &&
               nr < board.len() as i32 &&
               nc < board[0].len() as i32 {

                let nr = nr as usize;
                let nc = nc as usize;

                if board[nr][nc] != '#' {
                    Self::dfs(board, next_node, res, nr, nc);
                    
                }
            }
        }

      
        board[r][c] = ch;


        if next_node.children_count == 0 && next_node.word.is_none() {
    node.children[idx] = None;
    node.children_count -= 1;
}
    }

    fn insert(root: &mut TrieNode, word: String) {
    let mut node = root;

    for b in word.bytes() {
        let idx = (b - b'a') as usize;

        if node.children[idx].is_none() {
            node.children[idx] = Some(Box::new(TrieNode::default()));
            node.children_count += 1;
        }

        node = node.children[idx].as_mut().unwrap();
    }

    node.word = Some(word);
}
    }
