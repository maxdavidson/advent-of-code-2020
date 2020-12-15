const firstRegex = /^(?<color>[a-z ]+) bags? contain/;
const secondRegex = /(?<count>\d+) (?<color>[a-z ]+) bags?/g;

/**
 * @param {string} input
 */
function parseInput(input) {
  /** @type {Map<string, Map<string, number>>} */
  const data = new Map();

  for (const line of input.trim().split('\n')) {
    /** @type {Map<string, number>} */
    const counts = new Map();

    for (const match of line.matchAll(secondRegex)) {
      if (match.groups !== undefined) {
        const { color, count } = match.groups;
        counts.set(color, Number(count));
      }
    }

    const match = line.match(firstRegex);
    if (match === null || match.groups === undefined) {
      throw new TypeError(`Not matching: ${line}`);
    }

    const { color } = match.groups;
    data.set(color, counts);
  }

  return data;
}

/**
 * @template K, T
 * @param {(arg: K) => T} fn
 * @returns {(arg: K) => T}
 */
function memoize1(fn) {
  /** @type {Map<K, T>} */
  const cache = new Map();
  return key => {
    let value = cache.get(key);
    if (value === undefined) {
      value = fn(key);
      cache.set(key, value);
    }
    return value;
  };
}

/**
 * @param {string} input
 */
export function part1(input) {
  const data = parseInput(input);

  const containsShinyGold = memoize1(
    /**
     * @param {string} color
     * @returns {boolean}
     */
    color => {
      const colorCounts = data.get(color);
      return (
        colorCounts !== undefined &&
        (colorCounts.has('shiny gold') || Array.from(colorCounts.keys()).some(containsShinyGold))
      );
    },
  );

  let count = 0;
  for (const color of data.keys()) {
    if (containsShinyGold(color)) {
      count += 1;
    }
  }
  return count;
}

/**
 * @param {string} input
 */
export function part2(input) {
  const data = parseInput(input);

  const countBags = memoize1(
    /**
     * @param {string} color
     * @returns {number}
     */
    color => {
      const colorCounts = data.get(color);
      if (colorCounts === undefined) {
        return 0;
      }
      let sum = 0;
      for (const [nextColor, count] of colorCounts) {
        sum += count * (1 + countBags(nextColor));
      }
      return sum;
    },
  );

  return countBags('shiny gold');
}
