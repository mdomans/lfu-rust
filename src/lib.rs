use std::collections::HashMap;
use std::borrow::Borrow;


#[derive(Debug,Default)]
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
    ///
    /// Allows to check frequency for a key of given value
    ///
    /// ```
    /// use lfu::LFU;
    /// let mut lfu = LFU::new();
    /// lfu.get_frequency("a");
    /// lfu.insert("a".to_string(), "b".to_string());
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 0);
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 1);
    /// lfu.get("a");
    /// assert_eq!(lfu.get_frequency("a"), 2);
    /// ```
    pub fn get_frequency(&mut self, key: &str ) -> usize {
        if self.frequency_list.len() == 0 {
            self.frequency_list.push(Vec::with_capacity(0));
        } else {
            for (index, key_list) in self.frequency_list.iter().enumerate() {
                if key_list.iter().any(|e| e == key) {
                    return index
                }
            }
        }
        0
    }

    fn increment_frequency(&mut self, key: &str ) {
        let key_frequency = self.get_frequency(key);
        let key_list = &mut self.frequency_list[key_frequency];

        if !key_list.iter().any(|e| e == key) {
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

    fn zero_frequency(&mut self, key: &str) {
        let key_frequency = self.get_frequency(key);
        if key_frequency !=0 {
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
    /// let mut lfu = LFU::new();
    /// assert_eq!(lfu.get("a"), None);
    /// lfu.insert("a".to_string(), "b".to_string());
    /// assert_eq!(lfu.get("a"), Some(&"b".to_string()));
    /// ```
    pub fn get(&mut self, key: &str ) -> Option<&String> {
        if self.items.contains_key(key) {
            self.increment_frequency(key);
            return self.items.get(key)
        }
        None
    }
    ///
    /// Insert a value into LFU
    ///
    ///
    /// ```
    /// use lfu::LFU;
    /// let mut lfu = LFU::new();
    /// lfu.insert("a".to_string(), "b".to_string());
    /// ```
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
