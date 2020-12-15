/**
 * @param {string} input
 */
export function part1(input) {
  const nums = input.trim().split('\n').map(Number);

  for (let i0 = 0; i0 < nums.length - 1; i0 += 1) {
    for (let i1 = i0 + 1; i1 < nums.length; i1 += 1) {
      if (nums[i0] + nums[i1] === 2020) {
        return nums[i0] * nums[i1];
      }
    }
  }

  return -1;
}

/**
 * @param {string} input
 */
export function part2(input) {
  const nums = input.trim().split('\n').map(Number);

  for (let i0 = 0; i0 < nums.length - 2; i0 += 1) {
    for (let i1 = i0 + 1; i1 < nums.length - 1; i1 += 1) {
      for (let i2 = i1 + 1; i2 < nums.length; i2 += 1) {
        if (nums[i0] + nums[i1] + nums[i2] === 2020) {
          return nums[i0] * nums[i1] * nums[i2];
        }
      }
    }
  }

  return -1;
}
