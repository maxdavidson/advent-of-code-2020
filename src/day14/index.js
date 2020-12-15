const linesRegex = /^.*$/gm;

/**
 * @param {string} input
 */
function* lines(input) {
  for (const match of input.matchAll(linesRegex)) {
    yield match[0];
  }
}

const maskRegex = /mask = (?<mask>[01X]{36})$/;
const memRegex = /mem\[(?<address>\d+)\] = (?<value>\d+)$/;

/** @typedef {{ mask: string, memoryInit: [bigint, bigint][] }} Program */

/**
 * @param {string} input
 */
function* programs(input) {
  const linesIt = lines(input);

  let result = linesIt.next();
  /** @type {RegExpExecArray | null} */
  let match = null;

  while (
    !result.done &&
    (match = maskRegex.exec(result.value)) !== null &&
    match.groups !== undefined
  ) {
    const { mask } = match.groups;
    const memoryInit = [];

    while (
      !(result = linesIt.next()).done &&
      (match = memRegex.exec(result.value)) !== null &&
      match.groups !== undefined
    ) {
      const { address, value } = match.groups;
      memoryInit.push([BigInt(address), BigInt(value)]);
    }

    yield { mask, memoryInit };
  }
}

/**
 * @param {string} mask
 * @param {bigint} value
 */
function applyMask(mask, value) {
  for (let i = 0; i < mask.length; i += 1) {
    if (mask[i] === '1') {
      value |= 1n << BigInt(mask.length - i - 1);
    } else if (mask[i] === '0') {
      value &= ~(1n << BigInt(mask.length - i - 1));
    }
  }
  return value;
}

/**
 * @param {string} input
 */
export function part1(input) {
  /** @type {Map<bigint, bigint>} */
  const memory = new Map();

  for (const { mask, memoryInit } of programs(input)) {
    for (const [address, value] of memoryInit) {
      const maskedValue = applyMask(mask, value);
      memory.set(address, maskedValue);
    }
  }

  let sum = 0n;
  for (const value of memory.values()) {
    sum += value;
  }
  return sum;
}

/**
 * @param {string} mask
 * @param {number} index
 * @returns {Generator<string, void>}
 */
function* masks(mask, index = 0) {
  if (index === mask.length) {
    yield mask;
  } else if (mask[index] === '1') {
    yield* masks(mask, index + 1);
  } else if (mask[index] === '0') {
    yield* masks(`${mask.slice(0, index)}?${mask.slice(index + 1)}`, index + 1);
  } else if (mask[index] === 'X') {
    yield* masks(`${mask.slice(0, index)}0${mask.slice(index + 1)}`, index + 1);
    yield* masks(`${mask.slice(0, index)}1${mask.slice(index + 1)}`, index + 1);
  }
}

/**
 * @param {string} input
 */
export function part2(input) {
  /** @type {Map<bigint, bigint>} */
  const memory = new Map();

  for (const { mask, memoryInit } of programs(input)) {
    for (const newMask of masks(mask)) {
      for (const [address, value] of memoryInit) {
        const maskedAddress = applyMask(newMask, address);
        memory.set(maskedAddress, value);
      }
    }
  }

  let sum = 0n;
  for (const value of memory.values()) {
    sum += value;
  }
  return sum;
}
