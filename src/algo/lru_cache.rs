// use std::collections::HashMap;

// fn main() {
//     #[derive(Debug)]
//     struct LRUCache {
//         capacity: usize,
//         head: Option<usize>,
//         tail: Option<usize>,
//         store: HashMap<i32, Node>,
//         node_list: Vec<Node>,
//         length: usize,
//     }

//     #[derive(Clone, Debug)]
//     struct Node {
//         key: i32,
//         value: i32,
//         index: usize,
//         prev: Option<usize>,
//         next: Option<usize>,
//         evicted: bool,
//     }

//     impl Node {
//         fn new(key: i32, value: i32) -> Self {
//             Node {
//                 key,
//                 value,
//                 index: 0,
//                 prev: None,
//                 next: None,
//                 evicted: false,
//             }
//         }

//         fn _new_with_index(key: i32, value: i32, index: usize) -> Self {
//             Node {
//                 key,
//                 value,
//                 index,
//                 prev: None,
//                 next: None,
//                 evicted: false,
//             }
//         }

//         fn evict(mut self) -> Node {
//             self.evicted = true;
//             self
//         }
//     }

//     impl LRUCache {
//         fn new(capacity: i32) -> Self {
//             LRUCache {
//                 capacity: capacity as usize,
//                 head: None,
//                 tail: None,
//                 store: HashMap::new(),
//                 node_list: vec![],
//                 length: 0,
//             }
//         }

//         fn get(&mut self, key: i32) -> i32 {
//             let point_node = match self.store.get(&key) {
//                 Some(point) => {
//                     if point.evicted {
//                         return -1;
//                     }
//                     point
//                 }
//                 None => return -1,
//             };

//             let mut node = self.node_list[point_node.index].clone();

//             // if already head
//             if node.index == self.head.unwrap() {
//                 return node.value;
//             }

//             // updating nodes left and right // sus
//             if (node.index != self.head.unwrap()) && (node.index != self.tail.unwrap()) {
//                 let mut node_next = self.node_list[node.next.unwrap()].clone();
//                 let mut node_prev = self.node_list[node.prev.unwrap()].clone();
//                 // 4 3 2 | 3=node
//                 //  4 = node.next
//                 //  2 = node.prev
//                 node_prev.next = Some(node_next.index);
//                 node_next.prev = Some(node_prev.index);

//                 self.node_list[node.next.unwrap()] = node_next;
//                 self.node_list[node.prev.unwrap()] = node_prev;
//             }

//             if node.index == self.tail.unwrap() {
//                 //node next postaje tail
//                 let mut node_next = self.node_list[node.next.unwrap()].clone();

//                 node_next.prev = None;
//                 let next_index = node_next.index;
//                 self.node_list[next_index] = node_next;
//                 self.tail = Some(next_index);

//                 //  println!("{} ============ {}", &node.key, &self.tail.unwrap());
//             }

//             let mut old_head = self.node_list[self.head.unwrap()].clone();

//             let old_head_index = old_head.index.clone();
//             old_head.next = Some(node.index);
//             self.node_list[old_head_index] = old_head;

//             // setup new head
//             node.next = None;
//             node.prev = self.head;
//             self.head = Some(node.index);
//             self.node_list[node.index] = node.clone();

//             node.value
//         }

//         fn put(&mut self, key: i32, value: i32) {
//             let len = self.node_list.len();
//             if self.length == 0 && len == 0 {
//                 let mut node = Node::new(key, value);
//                 node.prev = Some(0);
//                 node.next = Some(0);
//                 node.index = 0;

//                 self.node_list.push(node.clone());
//                 self.head = Some(0);
//                 self.tail = Some(0);
//                 self.store.insert(key, node);
//                 self.length = self.length + 1;

//                 return;
//             }

//             if (self.length < self.capacity) && self.length > 0 && len > 0 {
//                 let mut node = Node::new(key, value);
//                 //   dbg!(node.clone());
//                 node.prev = self.head;
//                 node.index = len;

//                 self.node_list.push(node.clone());
//                 self.head = Some(len);
//                 self.node_list[len - 1].next = Some(len);

//                 match self.store.insert(key, node) {
//                     Some(_) => (),
//                     None => {
//                         self.length = self.length + 1;
//                         ()
//                     }
//                 }
//                 return;
//             }

//             if self.length >= self.capacity {
//                 let (is_old_key, mut node) = match self.store.insert(key, Node::new(key, value)) {
//                     Some(point_node) => {
//                         self.node_list[point_node.index].value = value;
//                         (true, self.node_list[point_node.index].clone())
//                     }
//                     None => (false, Node::new(key, value)),
//                 };

//                 if is_old_key {
//                     //move it to head and skip tail
//                     if node.index == self.head.unwrap() {
//                         self.store.insert(key, node);
//                         return;
//                     }

//                     let mut old_head = self.node_list[self.head.unwrap()].clone();
//                     old_head.next = Some(node.index);

//                     node.prev = Some(old_head.index);
//                     let temp_copy_node_next = node.next;
//                     node.next = None;
//                     self.head = Some(node.index);

//                     self.node_list[node.index] = node.clone();
//                     self.node_list[old_head.index] = old_head.clone();

//                     if self.tail.unwrap() == node.index {
//                         if node.prev.is_some() {
//                             self.tail = node.prev;
//                         } else {
//                             self.tail = temp_copy_node_next
//                         }
//                     }

//                     self.store.insert(key, node);
//                     return;
//                 }
//                 //
//                 let old_tail_index = self.tail.unwrap();
//                 let old_tail = self.node_list[old_tail_index].clone();

//                 let mut new_tail = self.node_list[old_tail.next.unwrap()].clone();
//                 new_tail.prev = None;

//                 self.tail = Some(new_tail.index);

//                 // update
//                 self.node_list[old_tail.index] = Node::new(old_tail.key, old_tail.value).evict();
//                 self.node_list[new_tail.index] = new_tail.clone();
//                 // do stuff
//                 node.prev = self.head;
//                 node.index = len;

//                 self.node_list.push(node.clone());
//                 self.node_list[self.head.unwrap()].next = Some(node.index);

//                 self.head = Some(node.index);

//                 self.store.insert(key, node);
//                 self.store.remove_entry(&old_tail.key);
//             }
//         }
//     }

//     // ["LRUCache","put","put","put","put","put","get","put","get","get","put","get","put","put","put","get","put","get","get","get","get","put","put","get","get","get","put","put","get","put","get","put","get","get","get","put","put","put","get","put","get","get","put","put","get","put","put","put","put","get","put","put","get","put","put","get","put","put","put","put","put","get","put","put","get","put","get","get","get","put","get","get","put","put","put","put","get","put","put","put","put","get","get","get","put","put","put","get","put","put","put","get","put","put","put","get","get","get","put","put","put","put","get","put","put","put","put","put","put","put"]
//     //,,[12,24],[5,18],[13],[7,23],[8],[12],[3,27],[2,12],[5],[2,9],[13,4],[8,18],[1,7],[6],[9,29],[8,21],[5],[6,30],[1,12],[10],[4,15],[7,22],[11,26],[8,17],[9,29],[5],[3,4],[11,30],[12],[4,29],[3],[9],[6],[3,4],[1],[10],[3,29],[10,28],[1,20],[11,13],[3],[3,12],[3,8],[10,9],[3,26],[8],[7],[5],[13,17],[2,27],[11,15],[12],[9,19],[2,15],[3,16],[1],[12,17],[9,1],[6,19],[4],[5],[5],[8,1],[11,7],[5,2],[9,28],[1],[2,2],[7,4],[4,22],[7,24],[9,26],[13,28],[11,26]]
//     let mut lru_cache = LRUCache::new(10);
//     lru_cache.put(10, 13);
//     lru_cache.put(3, 17);
//     lru_cache.put(6, 11);
//     lru_cache.put(10, 5);
//     lru_cache.put(9, 10);
//     lru_cache.get(13);
//     lru_cache.put(2, 19);
//     lru_cache.get(2);
//     lru_cache.get(3);
//     lru_cache.put(5, 25);
//     lru_cache.get(8);
//     lru_cache.put(9, 22);
//     lru_cache.put(5, 5);
//     lru_cache.put(1, 30);
//     lru_cache.get(11);
//     lru_cache.put(9, 12);
//     lru_cache.get(7);
//     lru_cache.get(5);
//     lru_cache.get(8);
//     lru_cache.get(9);
//     lru_cache.put(4, 30);
//     lru_cache.put(9, 3);
//     lru_cache.get(9);
//     lru_cache.get(10);
//     lru_cache.put(6, 14);
//     lru_cache.put(3, 1);
//     lru_cache.get(3);
//     lru_cache.put(10, 11);
//     lru_cache.get(8);
//     lru_cache.put(2, 14);
//     lru_cache.get(1);
//     lru_cache.get(5);
//     lru_cache.get(4);
//     lru_cache.put(11, 4);
//     lru_cache.put(12, 24)
// }

// // ,null,null,-1,null,-1,24,null,null,18,null,null,null,null,-1,null,null,18,null,null,-1,null,null,null,null,null,-1,null,null,24,null,4,29,30,null,12,-1,null,null,null,null,29,null,null,null,null,17,22,-1,null,null,null,24,null,null,null,20,null,null,null,29,-1,-1,null,null,null,null,-1,null,null,null,null,null,null,null]
// // ,null,null,-1,null,-1,24,null,null,18,null,null,null,null,-1,null,null,18,null,null,-1,null,null,null,null,null,18,null,null,-1,null,4,29,30,null,12,-1,null,null,null,null,29,null,null,null,null,17,22,18,null,null,null,-1,null,null,null,20,null,null,null,-1,18,18,null,null,null,null,20,null,null,null,null,null,null,null]
