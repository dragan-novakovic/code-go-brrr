mod algo;

fn main() {
    let list: Vec<Vec<i32>> = vec![
        [1, 3].to_vec(),
        [2, 6].to_vec(),
        [8, 10].to_vec(),
        [15, 18].to_vec(),
    ];

    let result = algo::merge_interval::merge_intervals(list);
    dbg!(result); //[[1,6],[8,10],[15,18]]
}
