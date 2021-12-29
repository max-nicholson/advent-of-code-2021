#[derive(Debug)]
pub struct WeightedTrie {
    pub height: usize,
    pub tree: Vec<usize>,
}

impl WeightedTrie {
    pub fn new(height: u32) -> Self {
        if height <= 1 {
            panic!("Must be positive height")
        }

        Self {
            height: height as usize,
            tree: vec![0; 2usize.pow(height + 1) - 1],
        }
    }

    pub fn insert(&mut self, input: &str) {
        if input.len() > self.height {
            panic!(
                "Trie was initialized to {} height, requires {} height",
                self.height,
                input.len()
            )
        }

        self.tree[0] += 1;
        let mut index = 0;
        for digit in input.chars() {
            index = match digit {
                '0' => Self::left_from(index),
                '1' => Self::right_from(index),
                _ => unimplemented!("Only binary characters expected"),
            };
            self.tree[index] += 1;
        }
    }

    pub fn left_from(i: usize) -> usize {
        2 * i + 1
    }

    pub fn right_from(i: usize) -> usize {
        2 * i + 2
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::WeightedTrie;

    #[test]
    fn test_trie() {
        let trie = WeightedTrie::new(2);
        assert_eq!(trie.height, 2);
        assert_eq!(trie.tree, vec![0; 7]);
    }

    #[test]
    #[should_panic(expected = "Must be positive height")]
    fn test_zero_height_trie() {
        WeightedTrie::new(0);
    }

    #[test]
    fn test_trie_insert() {
        let mut trie = WeightedTrie::new(2);
        trie.insert("10");
        assert_eq!(trie.tree, vec![1, 0, 1, 0, 0, 1, 0]);
        trie.insert("11");
        assert_eq!(trie.tree, vec![2, 0, 2, 0, 0, 1, 1]);
    }
}
