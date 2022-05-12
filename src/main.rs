use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct LRUCache {
        capacity: usize,
        head: Option<usize>,
        tail: Option<usize>,
        store: HashMap<i32, Node>,
        node_list: Vec<Node>,
        length: usize,
    }

    #[derive(Clone, Debug)]
    struct Node {
        key: i32,
        value: i32,
        index: usize,
        prev: Option<usize>,
        next: Option<usize>,
        evicted: bool,
    }

    impl Node {
        fn new(key: i32, value: i32) -> Self {
            Node {
                key,
                value,
                index: 0,
                prev: None,
                next: None,
                evicted: false,
            }
        }

        fn evict(mut self) -> Node {
            self.evicted = true;
            self
        }
    }

    impl LRUCache {
        fn new(capacity: i32) -> Self {
            LRUCache {
                capacity: capacity as usize,
                head: None,
                tail: None,
                store: HashMap::new(),
                node_list: vec![],
                length: 0,
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            let point_node = match self.store.get(&key) {
                Some(point) => {
                    if point.evicted {
                        return -1;
                    }
                    point
                }
                None => return -1,
            };

            let mut node = self.node_list[point_node.index].clone();

            // if already head
            if node.index == self.head.unwrap() {
                return node.value;
            }

            // updating nodes left and right // sus
            if (node.index != self.head.unwrap()) && (node.index != self.tail.unwrap()) {
                let mut node_next = self.node_list[node.next.unwrap()].clone();
                let mut node_prev = self.node_list[node.prev.unwrap()].clone();
                // 4 3 2 | 3=node
                //  4 = node.next
                //  2 = node.prev
                node_prev.next = Some(node_next.index);
                node_next.prev = Some(node_prev.index);

                self.node_list[node.next.unwrap()] = node_next;
                self.node_list[node.prev.unwrap()] = node_prev;
            }

            if node.index == self.tail.unwrap() {
                //node next postaje tail
                let mut node_next = self.node_list[node.next.unwrap()].clone();
                //  dbg!(node_next.clone());
                node_next.prev = None;
                let next_index = node_next.index;
                self.node_list[next_index] = node_next;
                self.tail = Some(next_index);

                //  println!("{} ============ {}", &node.key, &self.tail.unwrap());
            }

            let mut old_head = self.node_list[self.head.unwrap()].clone();

            let old_head_index = old_head.index.clone();
            old_head.next = Some(node.index);
            self.node_list[old_head_index] = old_head;

            // setup new head
            node.next = None;
            node.prev = self.head;
            self.head = Some(node.index);
            self.node_list[node.index] = node.clone();

            node.value
        }

        fn put(&mut self, key: i32, value: i32) {
            let len = self.node_list.len();
            if self.length == 0 && len == 0 {
                let mut node = Node::new(key, value);
                node.prev = Some(0);
                node.next = Some(0);
                node.index = 0;

                self.node_list.push(node.clone());
                self.head = Some(0);
                self.tail = Some(0);
                self.store.insert(key, node);
                self.length = self.length + 1;

                return;
            }

            if (self.length < self.capacity) && self.length > 0 && len > 0 {
                let mut node = Node::new(key.clone(), value);

                node.prev = self.head;
                node.index = len;

                self.node_list.push(node.clone());
                self.head = Some(len);
                self.node_list[len - 1].next = Some(len);

                match self.store.insert(key, node) {
                    Some(_) => (),
                    None => {
                        self.length = self.length + 1;
                        ()
                    }
                }
                return;
            }

            if self.length >= self.capacity {
                let mut node = Node::new(key, value);

                //
                let old_tail_index = self.tail.unwrap();
                let old_tail = self.node_list[old_tail_index].clone();

                let new_tail_index = match old_tail.clone().next {
                    Some(x) => x,
                    None => {
                        dbg!(old_tail.clone());
                        panic!("at the disco")
                    }
                };
                let mut new_tail = self.node_list[new_tail_index].clone();
                new_tail.prev = None;

                self.tail = Some(new_tail.index);

                // update
                self.node_list[old_tail.index] = Node::new(old_tail.key, old_tail.value).evict();
                self.node_list[new_tail.index] = new_tail.clone();
                // do stuff
                node.prev = self.head;
                node.index = len;

                // let mut old_head = self.node_list[self.head.unwrap()].clone();
                // old_head.next = Some(node.index);
                self.node_list.push(node.clone());

                self.node_list[self.head.unwrap()].next = Some(node.index); //bug

                self.head = Some(node.index);

                match self.store.insert(key, node) {
                    Some(_) => (),
                    None => {
                        self.store.remove_entry(&old_tail.key);
                        ()
                    }
                }
            }
        }
    }

    /*

        ["LRUCache","put","put","put","put","get","get","get","get","put","get","get","get","get","get"]
    [[3],[1,1],[2,2],[3,3],[4,4],[4],[3],[2],[1],[5,5],[1],[2],[3],[4],[5]]
             */

    let mut lru_cache = LRUCache::new(3);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    lru_cache.put(3, 3);
    lru_cache.put(4, 4);
    lru_cache.get(4);
    lru_cache.get(3); // 3 4 2
    lru_cache.get(2); // 2 3 4
                      // dbg!(&lru_cache);
    lru_cache.get(1); // - 1
                      //dbg!(&lru_cache);
    lru_cache.put(5, 5); // 5 2 3 (4)
                         // dbg!(&lru_cache);
    lru_cache.get(1);
    // dbg!(lru_cache);
    lru_cache.get(2); // 2 5 3
                      // dbg!(lru_cache);
    lru_cache.get(3); // -1 // 3 2 5
                      //dbg!(lru_cache);
    let y = lru_cache.get(4); // 4
                              //dbg!(y);
                              // lru_cache.get(5);
}
