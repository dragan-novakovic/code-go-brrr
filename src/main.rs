pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let result: Vec<i32> = nums
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (index, value)| {
            nums.iter().enumerate().for_each(|(inn_index, inner_val)| {
                if value + inner_val == target {
                    dbg!(value, inner_val);
                    acc.push(index as i32);
                    acc.push(inn_index as i32);
                }
            });
            acc
        });

    result
}

fn main() {
    let result = two_sum(vec![3, 2, 3, 5, 7, 10], 6);
    dbg!(result);
}
