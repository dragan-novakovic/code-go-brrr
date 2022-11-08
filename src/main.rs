mod algo;

fn main() {
    let list = vec![2, 1, 5, 3, 2, 7, 1, 3];

    let result = algo::binary_search::binary_search(list, 5);
    dbg!(result);
}
