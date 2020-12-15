const FLOOR = 0;
const EMPTY_SEAT = 1;
const OCCUPIED_SEAT = 2;

/**
 * @param {string} char
 */
function parseChar(char) {
  switch (char) {
    case '.':
      return FLOOR;
    case 'L':
      return EMPTY_SEAT;
    case '#':
      return OCCUPIED_SEAT;
    default:
      throw new Error(`Invalid char: ${char}`);
  }
}

/**
 * @param {string} line
 */
function parseLine(line) {
  return Array.from(line, parseChar);
}

/**
 * @param {string} input
 */
function parseInput(input) {
  const rows = input.trim().split('\n').map(parseLine);

  const rowCount = rows.length;
  const buffer = Uint8Array.from(rows.flat(1));
  const columnCount = buffer.length / rowCount;

  return { buffer, rowCount, columnCount };
}

/**
 * @param {ReturnType<typeof parseInput>} data
 */
function createView({ buffer, rowCount, columnCount }) {
  return Array.from({ length: rowCount }, (_, i) =>
    buffer.subarray(i * columnCount, (i + 1) * columnCount),
  );
}

/**
 * @param {ReturnType<typeof createView>} view
 * @param {number} x
 * @param {number} y
 */
function adjacentOccupiedSeatCount(view, x, y) {
  let count = 0;

  for (let dy = -1; dy <= 1; dy += 1) {
    const rowView = view[y + dy];
    if (rowView !== undefined) {
      for (let dx = -1; dx <= 1; dx += 1) {
        if (!(dy === 0 && dx === 0) && rowView[x + dx] === OCCUPIED_SEAT) {
          count += 1;
        }
      }
    }
  }

  return count;
}

/**
 * @template T
 * @param {ArrayLike<T>} a
 * @param {ArrayLike<T>} b
 */
function buffersAreEqual(a, b) {
  if (a.length !== b.length) {
    return false;
  }

  for (let i = 0; i < a.length; i += 1) {
    if (a[i] !== b[i]) {
      return false;
    }
  }

  return true;
}

/**
 * @param {number} char
 */
function formatChar(char) {
  switch (char) {
    case FLOOR:
      return '.';
    case EMPTY_SEAT:
      return 'L';
    case OCCUPIED_SEAT:
      return '#';
    default:
      throw new Error();
  }
}

/**
 * @param {ArrayLike<number>} line
 */
function formatLine(line) {
  return Array.from(line, formatChar).join('');
}

/**
 * @param {ReturnType<typeof createView>} view
 */
function printView(view) {
  return view.map(formatLine).join('\n');
}

/**
 * @param {string} input
 */
export function part1(input) {
  let sourceData = parseInput(input);
  let targetData = { ...sourceData, buffer: new Uint8Array(sourceData.buffer.length) };

  let sourceView = createView(sourceData);
  let targetView = createView(targetData);

  const { columnCount, rowCount } = sourceData;

  while (!buffersAreEqual(sourceData.buffer, targetData.buffer)) {
    for (let y = 0; y < rowCount; y += 1) {
      const sourceRowView = sourceView[y];
      const targetRowView = targetView[y];

      for (let x = 0; x < columnCount; x += 1) {
        const tile = sourceRowView[x];

        if (tile === EMPTY_SEAT && adjacentOccupiedSeatCount(sourceView, x, y) === 0) {
          targetRowView[x] = OCCUPIED_SEAT;
        } else if (tile === OCCUPIED_SEAT && adjacentOccupiedSeatCount(sourceView, x, y) >= 4) {
          targetRowView[x] = EMPTY_SEAT;
        } else {
          targetRowView[x] = tile;
        }
      }
    }

    // Swap buffers & views
    [sourceData, targetData] = [targetData, sourceData];
    [sourceView, targetView] = [targetView, sourceView];
  }

  return targetData.buffer.reduce((count, tile) => count + Number(tile === OCCUPIED_SEAT), 0);
}

/**
 * @param {ReturnType<typeof createView>} view
 * @param {number} x
 * @param {number} y
 */
function visibleOccupiedSeatCount(view, x, y) {
  let count = 0;

  for (let dy = -1; dy <= 1; dy += 1) {
    for (let dx = -1; dx <= 1; dx += 1) {
      if (!(dy === 0 && dx === 0)) {
        for (let i = 1; ; i += 1) {
          const x2 = x + i * dx;
          const y2 = y + i * dy;

          const tile = view[y2]?.[x2];

          if (tile !== FLOOR) {
            if (tile === OCCUPIED_SEAT) {
              count += 1;
            }

            break;
          }
        }
      }
    }
  }

  return count;
}

/**
 * @param {string} input
 */
export function part2(input) {
  let sourceData = parseInput(input);
  let targetData = { ...sourceData, buffer: new Uint8Array(sourceData.buffer.length) };

  let sourceView = createView(sourceData);
  let targetView = createView(targetData);

  const { columnCount, rowCount } = sourceData;

  while (!buffersAreEqual(sourceData.buffer, targetData.buffer)) {
    for (let y = 0; y < rowCount; y += 1) {
      const sourceRowView = sourceView[y];
      const targetRowView = targetView[y];

      for (let x = 0; x < columnCount; x += 1) {
        const tile = sourceRowView[x];

        if (tile === EMPTY_SEAT && visibleOccupiedSeatCount(sourceView, x, y) === 0) {
          targetRowView[x] = OCCUPIED_SEAT;
        } else if (tile === OCCUPIED_SEAT && visibleOccupiedSeatCount(sourceView, x, y) >= 5) {
          targetRowView[x] = EMPTY_SEAT;
        } else {
          targetRowView[x] = tile;
        }
      }
    }

    // Swap buffers & views
    [sourceData, targetData] = [targetData, sourceData];
    [sourceView, targetView] = [targetView, sourceView];
  }

  return targetData.buffer.reduce((count, tile) => count + Number(tile === OCCUPIED_SEAT), 0);
}
