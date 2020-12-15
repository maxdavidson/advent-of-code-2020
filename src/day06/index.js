/**
 * @param {string} input
 */
export function part1(input) {
  return input
    .trim()
    .split('\n\n')
    .map(group => new Set(group.split(/\s*/)).size)
    .reduce((a, b) => a + b, 0);
}

/**
 * @param {string} input
 */
export function part2(input) {
  return input
    .trim()
    .split('\n\n')
    .map(group => {
      const [head, ...tail] = group.split('\n');
      let intersection = new Set(head.split(''));
      for (const line of tail) {
        intersection = new Set(line.split('').filter(c => intersection.has(c)));
      }
      return intersection.size;
    })
    .reduce((a, b) => a + b, 0);
}
