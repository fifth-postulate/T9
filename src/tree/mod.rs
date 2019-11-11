use std::collections::HashMap;

use crate::pad::{digits_for, Digit, Digits};

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    root: Option<usize>,
}

impl Tree {
    pub fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    pub fn add<S>(&mut self, word: S)
    where
        S: Into<String>,
    {
        let word = word.into();
        let digits = digits_for(&word);
        let index = match self.root {
            Some(idx) => idx,
            None => {
                let idx = self.index_of_new_empty_node();
                self.root = Some(idx);
                idx
            }
        };

        self.add_at(index, digits, word);
    }

    fn index_of_new_empty_node(&mut self) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node::empty());
        index
    }

    fn add_at(&mut self, index: usize, digits: Digits, word: String) {
        if let Some(digit) = digits.head() {
            let sub_index = if let Some(sub_index) = self.nodes[index].index_of_subtree(*digit) {
                *sub_index
            } else {
                let sub_index = self.index_of_new_empty_node();
                self.nodes[index].add_child(*digit, sub_index);
                sub_index
            };
            self.add_at(sub_index, digits.tail(), word);
        } else {
            self.nodes[index].add(word);
        }
    }

    pub fn words_at<D>(&self, digits: D) -> Vec<String>
    where
        D: Into<Digits>,
    {
        let mut digits = digits.into();
        let mut current = self.root.as_ref();
        while let Some(index) = current {
            if let Some(digit) = digits.head() {
                current = self.nodes[*index].subtrees.get(digit);
                digits = digits.tail();
            } else {
                return self.nodes[*index].words.iter().cloned().collect();
            }
        }
        vec![]
    }
}

#[derive(Debug)]
struct Node {
    subtrees: HashMap<Digit, usize>,
    words: Vec<String>,
}

impl Node {
    fn empty() -> Self {
        Self {
            subtrees: HashMap::new(),
            words: Vec::new(),
        }
    }

    fn add(&mut self, word: String) {
        self.words.push(word);
    }

    fn add_child(&mut self, digit: Digit, index: usize) {
        self.subtrees.insert(digit, index);
    }

    fn index_of_subtree(&self, digit: Digit) -> Option<&usize> {
        self.subtrees.get(&digit)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adding_a_word_should_also_return_it() {
        let mut tree: Tree = Tree::empty();
        tree.add("rust");

        let actual = tree.words_at([Digit::Seven, Digit::Eight, Digit::Seven, Digit::Eight]);

        let expected: Vec<String> = vec![String::from("rust")];
        assert_eq!(actual, expected);
    }
}
