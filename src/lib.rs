use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};


#[derive(Debug)]
pub struct LFU {
    items: HashMap<String, String>,
    frequency_list: Vec<Vec<String>>,
    max_size: usize,
    key_count: usize,
}

impl LFU {
    pub fn new() -> Self {
        LFU {
            items: HashMap::new(),
            max_size: 0,
            key_count: 0,
            frequency_list: Vec::with_capacity(0)
        }
    }

    pub fn get_frequency(&self, key: &String ) -> usize {
        for (index, key_list) in self.frequency_list.iter().enumerate() {
            if key_list.contains(key){
                return index
            }
        }
        0
    }

    fn increment_frequency(&mut self, key: &String ) {
        // normally we would start the program with frequency_list having one empty list inside
        // but to keep with Rust spirit we handle emptiness and have 0 overhead

        let key_frequency = self.get_frequency(key);

        match self.frequency_list.get_mut(key_frequency){
            None => {
                self.frequency_list.push(vec![key.to_string()]);
            },
            Some(key_list) => {
                if !key_list.contains(key) {
                    key_list.push(key.to_string());
                } else {
                    key_list.retain(|lkey|lkey.ne(key));
                    match self.frequency_list.get_mut(key_frequency+1){
                        None => {
                            self.frequency_list.push(vec![key.to_string()]);
                        },
                        Some(key_list) => {
                            key_list.push(key.to_string())
                        }
                    }
                }
            }
        }
    }

    fn zero_frequency(&mut self, key: &String ) {
        match self.get_frequency(key) {
            0 => return,
            key_frequency => {
                match self.frequency_list.get_mut(key_frequency) {
                    Some(key_list) => {
                        key_list.retain(|lkey| lkey.ne(key))
                    },
                    None => panic!("returned frequency >0 for missing key"),
                }
            }
        }
    }

    pub fn get(&mut self, key: &String ) -> Option<&String> {
        match self.items.contains_key(key) {
            false => None,
            true => {
                self.increment_frequency(key);
                self.items.get(key)
            }
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.zero_frequency(key.borrow());
        self.items.insert(key, value)

    }

}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut lfu = LFU::new();
        lfu.insert("a".to_string(), "42".to_string());
        lfu.insert("b".to_string(), "43".to_string());
        lfu.insert("d".to_string(), "43".to_string());
        lfu.insert("c".to_string(), "44".to_string());
        assert_eq!(lfu.get(&"a".to_string()), Some(&"42".to_string()));
        assert_eq!(lfu.get(&"b".to_string()), Some(&"43".to_string()));
        assert_eq!(lfu.get(&"c".to_string()), Some(&"44".to_string()));
        assert_eq!(lfu.get(&"a".to_string()), Some(&"42".to_string()));
        assert_eq!(lfu.get(&"a".to_string()), Some(&"42".to_string()));
        assert_eq!(lfu.get(&"b".to_string()), Some(&"43".to_string()));
        assert_eq!(lfu.get(&"b".to_string()), Some(&"43".to_string()));
        assert_eq!(lfu.get(&"b".to_string()), Some(&"43".to_string()));
        assert_eq!(lfu.get(&"b".to_string()), Some(&"43".to_string()));
        assert_eq!(lfu.get(&"a".to_string()), Some(&"42".to_string()));
        assert_eq!(lfu.get(&"d".to_string()), Some(&"43".to_string()));
        print!("{:?}", lfu);

    }
}
