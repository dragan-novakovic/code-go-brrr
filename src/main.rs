mod algo;

fn main() {
    let list: Vec<i32> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let result = algo::trapping_rain_water::trapping_rain_water(list);
    dbg!(result); //[[1,6],[8,10],[15,18]]
}
