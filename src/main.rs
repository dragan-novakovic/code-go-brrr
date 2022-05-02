fn main() {
    struct LRUCache {
        capacity: i32,
        keys: Vec<i32>,
        prev_keys: Vec<i32>,
        values: Vec<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl LRUCache {
        fn new(capacity: i32) -> Self {
            LRUCache {
                capacity,
                keys: vec![],
                prev_keys: vec![],
                values: vec![],
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            match self.keys.iter().position(|k| k == &key) {
                Some(x) => {
                    self.prev_keys.push(key);
                    println!("STO ME ZAJ, {:?}", self.prev_keys);
                    self.values[x]
                }
                None => -1,
            }
        }

        fn put(&mut self, key: i32, value: i32) {
            if (self.keys.len() as i32) < self.capacity {
                self.keys.push(key);
                self.values.push(value);
            }
        }
    }

    let mut lRUCache = LRUCache::new(2);
    lRUCache.put(1, 1); // cache is {1=1}
    lRUCache.put(2, 2); // cache is {1=1, 2=2}
    lRUCache.get(1); // return 1
    lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    lRUCache.get(2); // returns -1 (not found)
    lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    lRUCache.get(1); // return -1 (not found)
    lRUCache.get(3); // return 3
    lRUCache.get(4); // return 4
}
