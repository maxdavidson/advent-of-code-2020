/**
 * @param {string} forest
 * @param {number} right
 * @param {number} down
 */
function* walkForest(forest, right, down) {
  const rows = forest.trim().split('\n');
  let y = 0;
  let x = 0;

  while (y < rows.length) {
    const row = rows[y];
    yield row[x];
    x = (x + right) % row.length;
    y = y + down;
  }
}

/**
 * @param {string} forest
 * @param {number} right
 * @param {number} down
 */
function countTrees(forest, right, down) {
  let count = 0;
  for (const place of walkForest(forest, right, down)) {
    if (place === '#') {
      count += 1;
    }
  }
  return count;
}

/**
 * @param {string} input
 */
export function part1(input) {
  return countTrees(input, 3, 1);
}

/**
 * @param {string} input
 */
export function part2(input) {
  return (
    countTrees(input, 1, 1) *
    countTrees(input, 3, 1) *
    countTrees(input, 5, 1) *
    countTrees(input, 7, 1) *
    countTrees(input, 1, 2)
  );
}
