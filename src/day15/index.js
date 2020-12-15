/**
 * @param {number[]} numbers
 * @param {number} index
 */
function getNumber(numbers, index) {
  if (index < numbers.length) {
    return numbers[index];
  }

  const NO_ENTRY = 2 ** 31 - 1; // SMI_MAX_VALUE
  const prevTurns = new Uint32Array(index + 1);
  prevTurns.fill(NO_ENTRY);

  for (let i = 0; i < numbers.length; ++i) {
    prevTurns[numbers[i]] = i;
  }

  let turn = numbers.length - 1;
  let number = numbers[turn];

  while (turn < index) {
    const prevTurn = prevTurns[number];
    prevTurns[number] = turn;
    number = prevTurn === NO_ENTRY ? 0 : turn - prevTurn;
    turn += 1;
  }

  return number;
}

/**
 * @param {string} input
 */
export function part1(input) {
  const numbers = input.split(',').map(Number);

  return getNumber(numbers, 2020 - 1);
}

/**
 * @param {string} input
 */
export function part2(input) {
  const numbers = input.split(',').map(Number);

  return getNumber(numbers, 30_000_000 - 1);
}
