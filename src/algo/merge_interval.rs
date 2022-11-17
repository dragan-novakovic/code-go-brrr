pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let clean_intervals =
        intervals
            .iter()
            .enumerate()
            .fold(vec![] as Vec<Vec<i32>>, |mut acc, (index, interval)| {
                let acc_len = acc.len();

                if acc_len == 0 {
                    acc.push(interval.clone());

                    return acc;
                }

                let prev_vec = acc.last().unwrap().clone();

                //refactor
                if prev_vec[1] > interval[0] {
                    //merge intervals
                    let first_element = if prev_vec[0] < interval[0] {
                        prev_vec[0]
                    } else {
                        interval[0]
                    };

                    let second_element = if prev_vec[1] < interval[1] {
                        interval[1]
                    } else {
                        prev_vec[1]
                    };

                    let merged_interval = vec![first_element, second_element];
                    acc[acc_len - 1] = merged_interval;
                }

                if prev_vec[1] < interval[0] {
                    acc.push(interval.clone())
                }

                return acc;
            });

    clean_intervals
}
