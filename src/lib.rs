//! Implementation of LFU protocol (not LRU) in Rust
//! Main focus is on performance and rustic approach rather than <insert language> in Rust.
//!
//! Implementation of http://dhruvbird.com/lfu.pdf
//!
//!
//! TODO:
//! * move to architecture using DataNode and FrequencyNode for O(1) complexity
//! * ... with proper memory management
//!
//!
//!

use bytes::Bytes;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
struct FrequencyNode {
    // frequency node value
    pub value: u32,
    items: Vec<String>,
    next: Option<Rc<RefCell<FrequencyNode>>>,
    // prev: Option<Weak<RefCell<FrequencyNode>>>
}

impl FrequencyNode {
    pub fn new(value: u32, next:Option<Rc<RefCell<FrequencyNode>>>) -> Self {
        FrequencyNode {
            value, items: vec![], next
        }
    }
}

/// original paper uses LFU Item but since this is private I see no reason for prefixing
#[derive(Debug, Default)]
struct Item {
    data: Bytes,
    parent: Rc<RefCell<FrequencyNode>>
}

impl Item {
    pub fn new(data: Bytes, parent: Rc<RefCell<FrequencyNode>>) -> Self {
        Item {data, parent}
    }
}

#[derive(Debug, Default)]
pub struct LFU {
    // main data storage, every cache can be usually thought of as a fixed size hashmap with extra method to evict certain keys when new value is added
    items: HashMap<String, Item>,
    frequency_head: Rc<RefCell<FrequencyNode>>,
    max_size: usize,
    current_size: usize,
}

impl LFU {
    pub fn new() -> Self {
        let frequency_head = FrequencyNode::new(0, None);
        LFU {
            items: HashMap::new(),
            max_size: 64,
            current_size: 0,
            frequency_head: Rc::new(RefCell::new(frequency_head)),
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
        let mut counter = 0;
        if self.items.contains_key(key){
            let mut frequency_node = self.frequency_head.clone();
            println!("{:?}", frequency_node);
            loop {
                if frequency_node.borrow().next.is_none() {
                    break
                }
                if frequency_node.borrow().items.iter().any(|f| f==key) {
                    break
                }
                let tmp = frequency_node.borrow().next.as_ref().unwrap().clone();
                frequency_node = tmp;
                counter += 1;
            }
        }
        counter
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
        let item = self.items.get_mut(key).unwrap();

        item.parent = if Rc::ptr_eq(&item.parent, &self.frequency_head) {

            let mut parent_frequency_node = item.parent.borrow_mut();
            // pop the key
            parent_frequency_node.items.retain(|x| x != key);

            match parent_frequency_node.next {
                Some(ref next_freq) => {
                    // push the key
                    next_freq.borrow_mut().items.push(key.to_owned());
                    item.parent.clone()
                },
                None => {
                    let mut next_freq = FrequencyNode::new(parent_frequency_node.value + 1, None);
                    // push the key

                    let ref_cell = Rc::new(RefCell::new(next_freq));
                    parent_frequency_node.next = Some(ref_cell.clone());

                    next_freq.items.push(key.to_owned());
                    ref_cell.clone()
                }
            }

        } else {
            item.parent.clone()
        };

        Some(&self.items.get(key).unwrap().data)
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
        let key_clone = key.clone();
        let previous = match self.items.insert(key, Item::new(value, self.frequency_head.clone())){
            Some(previous) => {
                Some(previous.data)
            },
            None => None
        };
        self.frequency_head.borrow_mut().items.push(key_clone);
        previous
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
        assert_eq!(lfu.get(&"a".to_string()), Some(&Bytes::from("42")));
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

    #[test]
    fn test_frequency() {
        let mut lfu = LFU::new().max_size(3);
        lfu.insert("a".to_string(), Bytes::from("42"));
        lfu.get("a");
        lfu.get("a");
        assert_eq!(lfu.get_frequency("a"), 2);
        println!("{:?}", lfu);
    }
}
