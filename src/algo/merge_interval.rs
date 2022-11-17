use std::collections::vec_deque;

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let clean_intervals =
        intervals
            .iter()
            .enumerate()
            .fold(vec![] as Vec<Vec<i32>>, |mut acc, (index, interval)| {
                if acc.len() == 0 {
                    acc.push(interval.clone());

                    return acc;
                }

                let prev_vec = acc.last().unwrap().clone();

                if prev_vec[1] > interval[0] {
                    // do merge intervals
                }

                return acc;
            });

    clean_intervals
}
