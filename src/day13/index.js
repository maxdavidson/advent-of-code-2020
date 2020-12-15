/**
 * @param {string} input
 */
function parseNotes(input) {
  const [lineA, lineB] = input.trim().split('\n');
  return {
    departureTimestamp: Number(lineA),
    busIds: lineB.split(',').map(Number),
  };
}

/**
 * @param {string} input
 */
export function part1(input) {
  const { departureTimestamp, busIds } = parseNotes(input);

  for (let timestamp = departureTimestamp; ; timestamp += 1) {
    for (const busId of busIds) {
      if (timestamp % busId === 0) {
        return (timestamp - departureTimestamp) * busId;
      }
    }
  }
}

/**
 * @param {string} input
 */
export function part2(input) {
  const { busIds } = parseNotes(input);

  let timestamp = 0;
  let stride = 1;

  for (let offset = 0; offset < busIds.length; ++offset) {
    const busId = busIds[offset];
    if (!Number.isNaN(busId)) {
      while ((timestamp + offset) % busId !== 0) {
        timestamp += stride;
      }
      stride *= busId;
    }
  }

  return timestamp;
}
