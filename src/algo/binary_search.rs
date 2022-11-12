pub fn binary_search(sorted_array: Vec<i32>, key: i32) -> i32 {
    let mut start_index = 0usize;
    let mut end_index = sorted_array.len() as usize - 1usize;

    while start_index <= end_index {
        let middle = (start_index + end_index) / 2;
        let middle = (middle as f64).floor() as usize;

        match sorted_array[middle] {
            value if value == key => {
                return key;
            }
            value if key > value => {
                start_index = middle;
            }
            value if key < value => {
                end_index = middle;
            }
            _ => {
                return -1;
            }
        }
    }

    // key wasn't found
    -1
}
