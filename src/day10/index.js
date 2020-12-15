/**
 * @param {string} input
 */
function parseInput(input) {
  const nums = input.trim().split('\n').map(Number);
  nums.push(0, Math.max(...nums) + 3);
  nums.sort((a, b) => a - b);
  return nums;
}

/**
 * @param {string} input
 */
export function part1(input) {
  const nodes = parseInput(input);

  /** @type {Map<number, number>} */
  const groups = new Map();

  for (let i = 0; i < nodes.length - 1; i += 1) {
    const diff = nodes[i + 1] - nodes[i];
    groups.set(diff, (groups.get(diff) ?? 0) + 1);
  }

  let product = 1;
  for (const value of groups.values()) {
    product *= value;
  }
  return product;
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
export function part2(input) {
  const nodes = parseInput(input);

  /** @type {Map<number, number[]>} */
  const successors = new Map();

  for (let i = 0; i < nodes.length - 3; i += 1) {
    const node = nodes[i];
    let j = i + 1;
    while (nodes[j] - node <= 3) {
      j += 1;
    }
    successors.set(node, nodes.slice(i + 1, j));
  }

  const pathCountFromNode = memoize1(
    /**
     * @param {number} node
     * @returns {number} count
     */
    node =>
      successors
        .get(node)
        ?.reduceRight((pathCount, nextNode) => pathCount + pathCountFromNode(nextNode), 0) ?? 1,
  );

  return pathCountFromNode(0);
}
