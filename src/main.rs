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
        fn new(capacity: usize) -> Self {
            LRUCache {
                capacity,
                head: None,
                tail: None,
                store: HashMap::new(),
                node_list: vec![],
                length: 0,
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            let list_len = self.node_list.len();
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

            // updating nodes left and right // sus
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

                //  println!("Heello 1. {:#?}", &self.store);

                let old_tail = self.node_list[self.tail.unwrap()].clone();
                let mut new_tail = self.node_list[old_tail.next.unwrap()].clone();
                new_tail.prev = None;

                self.tail = Some(new_tail.index);

                // update
                self.node_list[old_tail.index] = Node::new(old_tail.key, old_tail.value).evict();
                self.node_list[new_tail.index] = new_tail.clone();
                // do stuff
                node.prev = self.head;
                node.index = len;

                self.node_list.push(node.clone());
                self.head = Some(len);
                self.node_list[len - 1].next = Some(len);

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

    let mut lRUCache = LRUCache::new(3);
    lRUCache.put(1, 1);
    //dbg!(&lRUCache.store);
    lRUCache.put(2, 2);
    lRUCache.put(3, 3);
    lRUCache.put(4, 4);
    lRUCache.get(4); // 4 3 2
    lRUCache.get(3); // 3 4 2  || 2 4 3
                     // dbg!(&lRUCache.head);
                     // dbg!(&lRUCache.tail);
    dbg!(&lRUCache.node_list);
    lRUCache.get(2); // 2 3 4
    dbg!(&lRUCache.head);
    dbg!(&lRUCache.tail);
    dbg!(&lRUCache.node_list);
    lRUCache.get(1); // - 1
                     // dbg!(&lRUCache.tail);
                     // dbg!(&lRUCache.node_list);
                     // lRUCache.put(5, 5);
                     // // dbg!(&lRUCache.store);
                     // lRUCache.get(1);
                     // // dbg!(&lRUCache.store);
                     // lRUCache.get(2);
                     // // dbg!(&lRUCache.store);
                     // lRUCache.get(3); // -1
                     //                  // dbg!(&lRUCache.store);
                     // lRUCache.get(4); // 4
                     //                  //  dbg!(&lRUCache.store);
                     // lRUCache.get(5);
}
