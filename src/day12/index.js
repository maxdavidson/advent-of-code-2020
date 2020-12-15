const instructionsRegex = /([A-Z])(\d+)/g;

/**
 * @param {string} input
 */
function* instructions(input) {
  for (const [, char, value] of input.matchAll(instructionsRegex)) {
    yield { char, value: Number(value) };
  }
}

/**
 * @param {string} input
 */
export function part1(input) {
  let posX = 0;
  let posY = 0;
  let dirX = 1;
  let dirY = 0;

  for (const { char, value } of instructions(input)) {
    switch (char) {
      case 'N':
        posY += value;
        break;

      case 'S':
        posY -= value;
        break;

      case 'E':
        posX += value;
        break;

      case 'W':
        posX -= value;
        break;

      case 'L':
        switch (value) {
          case 90:
            [dirX, dirY] = [-dirY, dirX];
            break;
          case 180:
            [dirX, dirY] = [-dirX, -dirY];
            break;
          case 270:
            [dirX, dirY] = [dirY, -dirX];
            break;
        }
        break;

      case 'R':
        switch (value) {
          case 90:
            [dirX, dirY] = [dirY, -dirX];
            break;
          case 180:
            [dirX, dirY] = [-dirX, -dirY];
            break;
          case 270:
            [dirX, dirY] = [-dirY, dirX];
            break;
        }
        break;

      case 'F':
        posX += value * dirX;
        posY += value * dirY;
        break;
    }
  }

  return Math.abs(posX) + Math.abs(posY);
}

/**
 * @param {string} input
 */
export function part2(input) {
  let posX = 0;
  let posY = 0;
  let dirX = 10;
  let dirY = 1;

  for (const { char, value } of instructions(input)) {
    switch (char) {
      case 'N':
        dirY += value;
        break;

      case 'S':
        dirY -= value;
        break;

      case 'E':
        dirX += value;
        break;

      case 'W':
        dirX -= value;
        break;

      case 'L':
        switch (value) {
          case 90:
            [dirX, dirY] = [-dirY, dirX];
            break;
          case 180:
            [dirX, dirY] = [-dirX, -dirY];
            break;
          case 270:
            [dirX, dirY] = [dirY, -dirX];
            break;
        }
        break;

      case 'R':
        switch (value) {
          case 90:
            [dirX, dirY] = [dirY, -dirX];
            break;
          case 180:
            [dirX, dirY] = [-dirX, -dirY];
            break;
          case 270:
            [dirX, dirY] = [-dirY, dirX];
            break;
        }
        break;

      case 'F':
        posX += value * dirX;
        posY += value * dirY;
        break;
    }
  }

  return Math.abs(posX) + Math.abs(posY);
}
