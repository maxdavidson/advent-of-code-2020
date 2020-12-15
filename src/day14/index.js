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

/** @typedef {{ mask: string, memoryInit: [number, number][] }} Program */

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
      memoryInit.push([Number(address), Number(value)]);
    }

    yield { mask, memoryInit };
  }
}

/**
 * @param {string} mask
 * @returns {(value: number) => number}
 */
function createMaskApplier(mask) {
  const maskOn = BigInt(`0b${Array.from(mask, c => (c === '1' ? c : '0')).join('')}`);
  const maskOff = BigInt(`0b${Array.from(mask, c => (c === '0' ? c : '1')).join('')}`);
  return function applyMask(value) {
    return Number((BigInt(value) | maskOn) & maskOff);
  };
}

/**
 * @param {string} input
 */
export function part1(input) {
  /** @type {Map<number, number>} */
  const memory = new Map();

  for (const { mask, memoryInit } of programs(input)) {
    const applyMask = createMaskApplier(mask);
    for (const [address, value] of memoryInit) {
      const maskedValue = applyMask(value);
      memory.set(address, maskedValue);
    }
  }

  let sum = 0;
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
  /** @type {Map<number, number>} */
  const memory = new Map();

  for (const { mask, memoryInit } of programs(input)) {
    for (const newMask of masks(mask)) {
      const applyMask = createMaskApplier(newMask);
      for (const [address, value] of memoryInit) {
        const maskedAddress = applyMask(address);
        memory.set(maskedAddress, value);
      }
    }
  }

  let sum = 0;
  for (const value of memory.values()) {
    sum += value;
  }
  return sum;
}
