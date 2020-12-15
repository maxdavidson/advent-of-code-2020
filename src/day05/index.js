/**
 * @param {string} line
 */
function parseBoardingPass(line) {
  let row_from = 0;
  let row_to = 127;

  let column_from = 0;
  let column_to = 7;

  for (const c of line.split('')) {
    switch (c) {
      case 'B':
        row_from = Math.ceil((row_to + row_from) / 2);
        break;

      case 'F':
        row_to = Math.floor((row_to + row_from) / 2);
        break;

      case 'R':
        column_from = Math.ceil((column_from + column_to) / 2);
        break;

      case 'L':
        column_to = Math.floor((column_from + column_to) / 2);
        break;
    }
  }

  return [row_to, column_to];
}

/**
 * @param {string} input
 */
export function part1(input) {
  const lines = input.trim().split('\n');

  const boardingPassIds = lines.map(line => {
    const [row, column] = parseBoardingPass(line);
    return row * 8 + column;
  });

  return Math.max(...boardingPassIds);
}

/**
 * @param {string} input
 */
export function part2(input) {
  const lines = input.trim().split('\n');

  const boardingPasses = lines.map(parseBoardingPass);
  const boardingPassIds = new Set(boardingPasses.map(([row, column]) => row * 8 + column));
  const boardingPassRows = boardingPasses.map(([row]) => row);

  const firstRow = Math.min(...boardingPassRows);
  const lastRow = Math.max(...boardingPassRows);

  for (let row = firstRow + 1; row < lastRow; row += 1) {
    for (let col = 0; col < 8; col += 1) {
      const boardingPassId = row * 8 + col;
      if (
        !boardingPassIds.has(boardingPassId) &&
        (boardingPassIds.has(boardingPassId - 1) || boardingPassIds.has(boardingPassId + 1))
      ) {
        return boardingPassId;
      }
    }
  }

  throw new Error('No solution found!');
}
