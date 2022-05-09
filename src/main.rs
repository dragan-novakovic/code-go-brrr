use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct LRUCache {
        capacity: usize,
        head: Option<usize>,
        tail: Option<usize>,
        store: HashMap<i32, Node>,
        node_list: Vec<Node>,
    }

    #[derive(Clone, Debug)]
    struct Node {
        value: i32,
        index: usize,
        prev: Option<usize>,
        next: Option<usize>,
    }

    impl Node {
        fn new(value: i32) -> Self {
            Node {
                value,
                index: 0,
                prev: None,
                next: None,
            }
        }
    }

    impl LRUCache {
        fn new(capacity: usize) -> Self {
            LRUCache {
                capacity,
                head: None,
                tail: None,
                store: HashMap::new(),
                node_list: vec![],
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            let mut node = &mut self.store.get_mut(&key).unwrap();
            let list_len = self.node_list.len();

            // if already head
            if node.index == self.head.unwrap() {
                return node.value;
            }

            // if tail
            if node.index == self.tail.unwrap() {
                //node next postaje tail
                self.tail = Some(self.node_list[node.next.unwrap()].index);
            }

            // setup new head
            node.next = None;
            node.prev = self.head;

            let mut old_head = &mut self.node_list[self.head.unwrap()];
            old_head.next = Some(node.index);

            // updating nodes left and right
            if node.index >= 1 && node.index < list_len - 1 {
                let nodes = &mut self.node_list;
                let mut node_next = nodes[node.index + 1].clone();
                let mut node_prev = nodes[node.index - 1].clone();
                // 2 3 4
                // 2 PREV -> 4
                // 4 NEX -> 2

                node_prev.prev = Some(node_next.index);
                node_next.next = Some(node_prev.index);

                nodes[node.index + 1] = node_next;
                nodes[node.index - 1] = node_prev;
            }

            self.head = Some(node.index);

            node.value
        }

        fn put(&mut self, key: i32, value: i32) {
            let len = self.node_list.len();
            if len == 0 {
                let mut node = Node::new(value);
                node.prev = Some(0);
                node.next = Some(0);
                node.index = 0;

                self.node_list.push(node.clone());
                self.head = Some(0);
                self.tail = Some(0);
                self.store.insert(key, node);
            }

            if len < self.capacity && len > 0 {
                let mut node = Node::new(value);
                node.prev = self.head;
                node.index = len;

                self.node_list.push(node.clone());
                self.head = Some(len);
                self.node_list[len - 1].next = Some(len);
            }
        }
    }

    let mut lRUCache = LRUCache::new(2);
    lRUCache.put(1, 10); // cache is {1=1}
    lRUCache.put(2, 20); // cache is {1=1, 2=2}
    lRUCache.put(3, 30); // cache is {1=1, 2=2}

    lRUCache.get(1); // return 1
    dbg!(lRUCache.node_list);
    // lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    // lRUCache.get(2); // returns -1 (not found)
    // lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    // lRUCache.get(1); // return -1 (not found)
    // lRUCache.get(3); // return 3
    // lRUCache.get(4); // return 4
}
