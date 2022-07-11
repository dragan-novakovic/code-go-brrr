pub fn _sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }

    arr
}

//2, 1, 5, 3, 2, 7, 1, 3
