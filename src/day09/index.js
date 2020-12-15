/**
 * @param {number[]} numbers
 * @param {number} preambleLength
 */
function findFirstInvalidNumber(numbers, preambleLength) {
  loop: for (let i = 0; i < numbers.length - (preambleLength + 1); i += 1) {
    const last = numbers[i + preambleLength];

    for (let aIndex = i; aIndex < i + preambleLength - 1; aIndex += 1) {
      const a = numbers[aIndex];
      for (let bIndex = aIndex + 1; bIndex < i + preambleLength; bIndex += 1) {
        const b = numbers[bIndex];
        if (a !== b && a + b === last) {
          continue loop;
        }
      }
    }

    return last;
  }
}

/**
 * @param {string} input
 * @param {number} preambleLength
 */
export function part1(input, preambleLength) {
  const numbers = input.trim().split('\n').map(Number);

  return findFirstInvalidNumber(numbers, preambleLength);
}

/**
 * @param {string} input
 * @param {number} preambleLength
 */
export function part2(input, preambleLength) {
  const numbers = input.trim().split('\n').map(Number);

  let sum = 0;
  const sums = numbers.map(val => (sum += val));

  const firstInvalidNumber = findFirstInvalidNumber(numbers, preambleLength);

  if (firstInvalidNumber === undefined) {
    throw new Error('No first invalid number found');
  }

  for (let i = 0; i < numbers.length - 1; i += 1) {
    let start = i;
    let end = numbers.length - 1;

    while (start < end) {
      const mid = Math.floor((start + end) / 2);
      const sum = sums[mid] - sums[start];

      if (sum < firstInvalidNumber) {
        start = mid + 1;
      } else if (sum > firstInvalidNumber) {
        end = mid - 1;
      } else {
        const window = numbers.slice(i, mid + 1);
        return Math.min(...window) + Math.max(...window);
      }
    }
  }

  throw new Error('No solution found');
}
