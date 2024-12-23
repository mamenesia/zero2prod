use std::collections::HashSet;

fn main() {
    println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let nums_set: HashSet<_> = nums.into_iter().collect();
    let mut result = vec![];

    let mut sorted_nums: Vec<_> = nums_set.into_iter().collect();
    sorted_nums.sort();

    for &num in &sorted_nums {
        if num == i32::MIN || !sorted_nums.contains(&(num - 1)) {
            let mut range = 1;
            while num < i32::MAX && sorted_nums.contains(&(num + range)) {
                range += 1;
            }

            if range > 1 {
                result.push(format!("{:?}->{:?}", num, num + range - 1));
            } else {
                result.push(num.to_string())
            }
        }
    }
    result
}
