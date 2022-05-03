use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct LRUCache {
        capacity: i32,
        head: Box<Option<Node>>,
        tail: Box<Option<Node>>,
        store: HashMap<i32, Node>,
    }

    #[derive(Clone, Debug)]
    struct Node {
        value: i32,
        prev: Box<Option<Node>>,
        next: Box<Option<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Self {
            Node {
                value,
                prev: Box::new(None),
                next: Box::new(None),
            }
        }
    }

    impl LRUCache {
        fn new(capacity: i32) -> Self {
            LRUCache {
                capacity,
                head: Box::new(None),
                tail: Box::new(None),
                store: HashMap::new(),
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            2
        }

        fn put(&mut self, key: i32, value: i32) {
            if (self.store.len() as i32) < self.capacity {
                if (*self.head.clone()).is_none() {
                    let mut head = Node::new(value);
                    let tail = Node::new(value);

                    head.next = Box::new(Some(tail));
                    (*head.next).unwrap().prev = Box::new(Some(head));

                    self.head = Box::new(Some(head));
                    self.tail = head.next;
                }

                let mut new_head = Node::new(value);
                let mut old_head = (*self.head.clone()).unwrap();

                new_head.next = self.head.clone();
                (*new_head.next).unwrap().prev = Box::new(Some(new_head));

                self.head = Box::new(Some(new_head.clone()));

                self.store.insert(key, new_head);
            } else {
            }
        }
    }

    let mut lRUCache = LRUCache::new(2);
    lRUCache.put(1, 1); // cache is {1=1}
                        // lRUCache.put(2, 2); // cache is {1=1, 2=2}
                        // lRUCache.get(1); // return 1
                        // lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
                        // lRUCache.get(2); // returns -1 (not found)
                        // lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
                        // lRUCache.get(1); // return -1 (not found)
                        // lRUCache.get(3); // return 3
                        // lRUCache.get(4); // return 4
}
