//! Implementation of LFU protocol (not LRU) in Rust
//! Main focus is on performance and rustic approach rather than <insert language> in Rust.
//!
//! Implementation of http://dhruvbird.com/lfu.pdf
//!
//!
//! TODO:
//! * cache size counting
//! * cache over-size evictions
//! * ... evictions?
//!
//!
//!

use bytes::Bytes;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct LFU {
    // main data storage, every cache can be usually thought of as a fixed size hashmap with extra method to evict certain keys when new value is added
    items: HashMap<String, Bytes>,
    frequency_list: Vec<Vec<String>>,
    max_size: usize,
    current_size: usize,
}

impl LFU {
    pub fn new() -> Self {
        LFU {
            items: HashMap::new(),
            max_size: 64,
            current_size: 0,
            frequency_list: Vec::with_capacity(0),
        }
    }
    ///
    /// Builder for max_size, only outside-configurable value for cache
    ///
    /// ```
    /// use lfu::LFU;
    /// let lfu = LFU::new().max_size(1024);
    /// ```
    ///
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }
    ///
    /// Allows to check frequency for a key of given value
    ///
    /// ```
    /// use lfu::LFU;
    /// use bytes::Bytes;
    /// let mut lfu = LFU::new();
    /// lfu.get_frequency("a");
    /// lfu.insert("a".to_string(), Bytes::from("b"));
    /// assert_eq!(lfu.get_frequency("a"), 0);
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 0);
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 1);
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 2);
    /// ```
    pub fn get_frequency(&mut self, key: &str) -> usize {
        if self.frequency_list.is_empty() {
            self.frequency_list.push(Vec::with_capacity(0));
        } else {
            for (index, key_list) in self.frequency_list.iter().enumerate() {
                if key_list.iter().any(|e| e == key) {
                    return index;
                }
            }
        }
        0
    }

    fn increment_frequency(&mut self, key: &str) {
        let key_frequency = self.get_frequency(key);
        let key_list = &mut self.frequency_list[key_frequency];

        if !key_list.iter().any(|e| e == key) {
            key_list.push(key.to_string());
        } else {
            key_list.retain(|lkey| lkey.ne(key));
            match self.frequency_list.get_mut(key_frequency + 1) {
                None => {
                    self.frequency_list.push(vec![key.to_string()]);
                }
                Some(key_list) => key_list.push(key.to_string()),
            }
        }
    }

    fn zero_frequency(&mut self, key: &str) {
        let key_frequency = self.get_frequency(key);
        if key_frequency != 0 {
            if let Some(key_list) = self.frequency_list.get_mut(key_frequency) {
                key_list.retain(|lkey| lkey.ne(key))
            } else {
                panic!("returned frequency >0 for missing key")
            }
        }
    }
    ///
    /// Get a Some(value) or None for a given key
    ///
    ///
    /// ```
    /// use lfu::LFU;
    /// use bytes::Bytes;
    /// let mut lfu = LFU::new();
    /// assert_eq!(lfu.get("a"), None);
    /// lfu.insert("a".to_string(), Bytes::from("b"));
    /// assert_eq!(lfu.get("a"), Some(&Bytes::from("b")));
    /// ```
    pub fn get(&mut self, key: &str) -> Option<&Bytes> {
        if !self.items.contains_key(key) {
            return None;
        }
        self.increment_frequency(key);
        self.items.get(key)
    }
    ///
    /// Insert a value into LFU
    ///
    ///
    /// ```
    /// use lfu::LFU;
    /// use bytes::Bytes;
    /// let mut lfu = LFU::new();
    /// lfu.insert("a".to_string(), Bytes::from("b"));
    /// ```
    pub fn insert(&mut self, key: String, value: Bytes) -> Option<Bytes> {
        if self.current_size + value.len() >= self.max_size {
            if let Some(freed) = self.evict(value.len()) {
                if freed >= value.len() {
                    return self.insert_raw(key, value);
                }
            }
        } else {
            return self.insert_raw(key, value);
        }
        None
    }

    fn insert_raw(&mut self, key: String, value: Bytes) -> Option<Bytes> {
        self.zero_frequency(key.borrow());
        self.current_size += value.len();
        self.items.insert(key, value)
    }

    ///
    /// Function to evict data, takes size required as an argument
    ///
    fn evict(&mut self, space_needed: usize) -> Option<usize> {
        let mut space_freed = 0;
        for key_list in self.frequency_list.iter() {
            println!("{:?}", key_list);
            for key in key_list {
                println!("B");
                if let Some(value) = self.items.remove(key) {
                    space_freed += value.len();
                    if space_freed >= space_needed {
                        return Some(space_freed);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate quickcheck;

    use crate::*;
    use bytes::Bytes;

    #[test]
    fn it_works() {
        let mut lfu = LFU::new();
        lfu.insert("a".to_string(), Bytes::from("42"));
        lfu.insert("b".to_string(), Bytes::from("43"));
        lfu.insert("d".to_string(), Bytes::from("43"));
        lfu.insert("c".to_string(), Bytes::from("44"));
        assert_eq!(lfu.get(&"a".to_string()), Some(&Bytes::from("42")));
        assert_eq!(lfu.get(&"b".to_string()), Some(&Bytes::from("43")));
        assert_eq!(lfu.get(&"c".to_string()), Some(&Bytes::from("44")));
        assert_eq!(lfu.get(&"a".to_string()), Some(&Bytes::from("42")));
        assert_eq!(lfu.get(&"a".to_string()), Some(&Bytes::from("42")));
        assert_eq!(lfu.get(&"b".to_string()), Some(&Bytes::from("43")));
        assert_eq!(lfu.get(&"b".to_string()), Some(&Bytes::from("43")));
        assert_eq!(lfu.get(&"b".to_string()), Some(&Bytes::from("43")));
        assert_eq!(lfu.get(&"b".to_string()), Some(&Bytes::from("43")));
        assert_eq!(lfu.get(&"a".to_string()), Some(&Bytes::from("42")));
        assert_eq!(lfu.get(&"d".to_string()), Some(&Bytes::from("43")));
        print!("{:?}", lfu);
    }
    #[test]
    fn test_max_size() {
        let lfu = LFU::new().max_size(1000);
        assert_eq!(lfu.max_size, 1000);
    }

    #[test]
    fn test_evictions() {
        let mut lfu = LFU::new().max_size(3);
        lfu.insert("a".to_string(), Bytes::from("42"));
        lfu.insert("b".to_string(), Bytes::from("43"));
        println!("{:?}", lfu);
    }
}
