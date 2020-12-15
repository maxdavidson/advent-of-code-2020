/**
 * @param {string} input
 */
function* entries(input) {
  for (const [_, a, b, char, word] of input.matchAll(/^(\d+)-(\d+) (\w): (\w+)$/gm)) {
    yield { a: Number.parseInt(a), b: Number.parseInt(b), char, word };
  }
}

/**
 * @param {string} input
 */
export function part1(input) {
  let count = 0;
  for (const { a, b, char, word } of entries(input)) {
    let charCount = 0;
    let index = -1;
    while ((index = word.indexOf(char, index + 1)) !== -1) {
      charCount += 1;
    }
    if (a <= charCount && charCount <= b) {
      count += 1;
    }
  }
  return count;
}

/**
 * @param {string} input
 */
export function part2(input) {
  let count = 0;
  for (const { a, b, char, word } of entries(input)) {
    if (Number(word[a - 1] === char) + Number(word[b - 1] === char) === 1) {
      count += 1;
    }
  }
  return count;
}
