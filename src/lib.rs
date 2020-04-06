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
            frequency_list: Vec::with_capacity(10),
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
        let key_frequency = self.get_frequency(key);

        if self.frequency_list.len() <= key_frequency {
            let mut target_list = Vec::new();
            target_list.push(key.to_string());
            self.frequency_list.push(target_list);

        } else {
            let key_list = &mut self.frequency_list[key_frequency];
            if !key_list.contains(key){
                key_list.push(key.to_string());
            } else {
                // remove key from lower list
                key_list.retain(|lkey|lkey.ne(key));
                // move key to upper list
                // create new list if missing
                if self.frequency_list.len() <= key_frequency + 1 {
                    let mut target_list = Vec::new();
                    target_list.push(key.to_string());
                    self.frequency_list.push(target_list);
                } else {
                    self.frequency_list[key_frequency+1].push(key.to_string());
                }
            }
        }

    }

    fn zero_frequency(&mut self, key: &String ) {
        let key_frequency = self.get_frequency(key);
        if key_frequency == 0 {
            return
        }
        let key_list = &mut self.frequency_list[key_frequency as usize];
        key_list.retain(|lkey|lkey.ne(key));
    }

    pub fn get(&mut self, key: &String ) -> Option<&String> {
        if !self.items.contains_key(k) {
            return None
        }
        // because increment is slow
        self.increment_frequency(key);
        self.items.get(key)
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
