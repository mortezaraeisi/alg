/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
  const m = new Map();
  for (let i = 0; i < nums.length; ++i) {
    const x = nums[i];
    const t = target - x;
    if (m.has(t)) {
      return [i, m.get(t)];
    }
    m.set(x, i);
  }
};