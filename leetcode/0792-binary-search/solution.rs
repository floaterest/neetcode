fn binary_search<T: Ord>(slice: &[T], target: T) -> Option<usize> {
    slice.binary_search(&target).ok()
}

impl Solution {

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    binary_search(&nums, target).map_or(-1, |i| i as i32)
}

}
