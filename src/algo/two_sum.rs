pub fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first_num = 99;
    let mut second_num = 99;

    'outer: for (i, v) in nums.iter().enumerate() {
        first_num = i as i32;
        dbg!(i);

        if i + 1 <= nums.len() {
            for j in i + 1..nums.len() {
                if v + nums[j] == target {
                    dbg!(j);
                    second_num = j as i32;
                    break 'outer;
                }
            }
        }
    }

    vec![first_num, second_num]
}
