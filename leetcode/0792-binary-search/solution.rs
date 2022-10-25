
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // if let Ok(i) = nums.binary_search(&target) { i as i32 } else { -1 }
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }
        return -1;

    }
}




