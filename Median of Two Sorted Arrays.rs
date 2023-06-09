impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
       let mut nums = [nums1, nums2].concat();

       nums.sort();

       let middle_index = (nums.len() - 1) >> 1;

       let median = (nums[middle_index] as f64 + nums[middle_index + (1 & nums.len() - 1)] as f64) / 2.0;

       median
    }
}
