use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CharTree {
    root: Node
}

#[derive(Serialize, Deserialize)]
struct Node {
    count: u32,
    children: HashMap<char, Node>,
}

impl CharTree {
    pub fn new() -> CharTree {
        CharTree {
            root: Node {
                count: 0,
                children: HashMap::new()
            }
        }
    }

    pub fn from_string(string: String) -> CharTree {
        serde_json::from_str(&string).unwrap()
    }

    pub fn train(&mut self, string: String) {
        let chars: Vec<char> = string.chars().collect();
        for i in 0..chars.len() {
            self.put(&chars[i..]);
        }
    }

    fn put(&mut self, key: &[char]) {
        Self::put_recur(&mut self.root, key);
    }

    fn put_recur(node: &mut Node, key: &[char]) {
        if key.is_empty() {
            return;
        }
        node.count += 1;
        let next = node.children.get_mut(&key[0]);
        match next {
            Some(n) => Self::put_recur(n, &key[1..]),
            _ => {
                node.children.insert(key[0], Node {
                    count: 0,
                    children: HashMap::new()
                });
            }
        }
    }

    pub fn depth(&self, key: &[char]) -> u32 {
        Self::depth_recur(&self.root, key)
    }

    fn depth_recur(node: &Node, key: &[char]) -> u32 {
        if key.len() == 0 {
            return 0;
        }
        match node.children.get(&key[0]) {
            Some(n) => Self::depth_recur(&n, &key[1..]) + 1,
            _ => 0
        }
    }

    pub fn get_weirdness(&self, string: &str) -> Vec<u32> {
        let mut weirdness = vec![];
        let chars: Vec<char> = string.chars().collect();
        for i in 0..chars.len() {
            weirdness.push(self.depth(&chars[i..]));
        }
        weirdness
    }
}

impl ToString for CharTree {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}