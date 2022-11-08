pub fn sort(arr: Vec<i32>) -> Vec<i32> {
    let mut heap: Vec<i32> = vec![arr[0]];

    fn parentIndex(i: f64) -> usize {
        let parentIndex = i - 1f64 / 2f64;
        parentIndex.floor() as usize
    }

    for index in 1..arr.len() {
        while arr[index] < arr[parentIndex(index as f64)] {
            heap.swap(index, parentIndex(index as f64));
        }
    }

    vec![]
}
