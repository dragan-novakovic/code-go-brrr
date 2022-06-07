pub fn _sort(mut list: Vec<i32>) -> Vec<i32> {
    for (mut i, _val) in list.clone().iter_mut().enumerate() {
        if i > 0 {
            let mut j = i - 1;
            while (j >= 0) && (list[j] > list[i]) {
                list[j + 1] = list[i];
                j = j - 1;
            }
        }
    }

    dbg!(&list.clone());
    list.clone()
}
