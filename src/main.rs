mod algo;

fn main() {
    let list = vec![1, 3, 5, 7, 8, 11, 14, 17, 22, 26, 31];

    let result = algo::binary_search::binary_search(list, 5);
    dbg!(result);
}
