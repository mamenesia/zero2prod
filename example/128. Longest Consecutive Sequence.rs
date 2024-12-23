fn main() {}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    //Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
    //You must write an algorithm that runs in O(n) time.

    let num_set: HashSet<_> = nums.into_iter().collect();
    let mut longest = 0;

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut length = 1;
            while num_set.contains(&(num + length)) {
                length += 1;
            }
            longest = longest.max(length);
        }
    
    }

    longest
}
