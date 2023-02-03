impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut dic = std::collections::HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            let v = nums[i];
            let exp = target - v;
            if let Some(ix) = dic.get(&exp) {
                return vec![*ix, i as i32];
            }
            dic.insert(v, i as i32);
        }
        vec![0, 0]
    }
}