pub fn _sort(list: Vec<i32>) -> Vec<i32> {
    let out = vec![];

    let mut current: i32;
    for (i, val) in list.iter().enumerate() {
        if i > 0 {
            current = list[i];
            dbg!(i);
            let j = i - 1;
            while j >= 0 && list[j] < list[i] {
                //
            }
        }
    }

    dbg!(&out);
    out
}
