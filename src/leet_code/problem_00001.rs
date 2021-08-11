struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            map.insert(num, index);
        }
        let mut result: Vec<i32> = Vec::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(i) = map.get(&(target - num)) {
                if index != *i {
                    result.push(index as i32);
                    result.push(*i as i32);
                    break;
                }
            }
        }
        result
    }
}

#[test]
fn test_two_sum() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let result: Vec<i32> = vec![0, 1];
    assert_eq!(Solution::two_sum(nums, 9), result);
}
