use std::{collections::HashMap, vec};

struct Solution {}

impl Solution {
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len() - 1;
        for i in 0..=len {
            for j in (i + 1)..=len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        // Returns this is no solution is found
        vec![-1, -1]
    }

    pub fn two_sum_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let x = target - v;
            match map.get(&x) {
                Some(j) => return vec![i as i32, *j],
                None => map.insert(*v, i as i32),
            };
        }

        vec![-1, -1]
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn example_1_brute_force() {
        let res = Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9);
        assert_eq!(res, [0, 1]);
    }

    #[test]
    fn example_2_brute_force() {
        let res = Solution::two_sum_brute_force(vec![3, 2, 4], 6);
        assert_eq!(res, [1, 2]);
    }

    #[test]
    fn example_3_brute_force() {
        let res = Solution::two_sum_brute_force(vec![3, 3], 6);
        assert_eq!(res, [0, 1]);
    }

    #[test]
    fn example_1_hash_map() {
        let res = Solution::two_sum_map(vec![2, 7, 11, 15], 9);
        assert_eq!(res, [1, 0]);
    }

    #[test]
    fn example_2_hash_map() {
        let res = Solution::two_sum_map(vec![3, 2, 4], 6);
        assert_eq!(res, [2, 1]);
    }

    #[test]
    fn example_3_hash_map() {
        let res = Solution::two_sum_map(vec![3, 3], 6);
        assert_eq!(res, [1, 0]);
    }
}
