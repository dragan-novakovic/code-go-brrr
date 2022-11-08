pub fn binary_search(sorted_array: Vec<i32>, key: i32) -> usize {
    let mut start = 0usize;
    let mut end = sorted_array.len() as usize - 1usize;

    while start <= end {
        let middle = (start + end) / 2;
        let middle = (middle as f64).floor() as usize;

        if sorted_array[middle] == key {
            // found the key
            return middle;
        } else if sorted_array[middle] < key {
            // continue searching to the right
            start = middle + 1;
        } else {
            // search searching to the left
            end = middle - 1;
        }
    }
    // key wasn't found
    return 1;
}
