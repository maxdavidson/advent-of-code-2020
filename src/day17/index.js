/**
 * @param {string} input
 */
export function part1(input) {
  let activeCubes = new Map();

  for (const [y, line] of input.trim().split('\n').entries()) {
    for (const [x, char] of line.split('').entries()) {
      if (char === '#') {
        const cube = [x, y, 0];
        const cubeKey = cube.join(',');
        activeCubes.set(cubeKey, cube);
      }
    }
  }

  for (let i = 0; i < 6; i += 1) {
    const nextActiveCubes = new Map();
    const inactiveNeighborCubes = new Map();

    for (const [cubeKey, cube] of activeCubes) {
      const [x, y, z] = cube;

      let activeNeighborCount = 0;

      for (let dx = -1; dx <= 1; dx += 1) {
        for (let dy = -1; dy <= 1; dy += 1) {
          for (let dz = -1; dz <= 1; dz += 1) {
            if (dx !== 0 || dy !== 0 || dz !== 0) {
              const neighborCubeKey = `${x + dx},${y + dy},${z + dz}`;

              if (activeCubes.has(neighborCubeKey)) {
                activeNeighborCount += 1;
              } else if (!inactiveNeighborCubes.has(neighborCubeKey)) {
                const neighborCube = [x + dx, y + dy, z + dz];
                inactiveNeighborCubes.set(neighborCubeKey, neighborCube);
              }
            }
          }
        }
      }

      if (activeNeighborCount === 2 || activeNeighborCount === 3) {
        nextActiveCubes.set(cubeKey, cube);
      }
    }

    for (const [cubeKey, cube] of inactiveNeighborCubes) {
      const [x, y, z] = cube;

      let activeNeighborCount = 0;

      for (let dx = -1; dx <= 1; dx += 1) {
        for (let dy = -1; dy <= 1; dy += 1) {
          for (let dz = -1; dz <= 1; dz += 1) {
            if (dx !== 0 || dy !== 0 || dz !== 0) {
              const neighborCube = `${x + dx},${y + dy},${z + dz}`;

              if (activeCubes.has(neighborCube)) {
                activeNeighborCount += 1;
              }
            }
          }
        }
      }

      if (activeNeighborCount === 3) {
        nextActiveCubes.set(cubeKey, cube);
      }
    }

    activeCubes = nextActiveCubes;
  }

  return activeCubes.size;
}

/**
 * @param {string} input
 */
export function part2(input) {
  let activeCubes = new Map();

  for (const [y, line] of input.trim().split('\n').entries()) {
    for (const [x, char] of line.split('').entries()) {
      if (char === '#') {
        const cube = [x, y, 0, 0];
        const cubeKey = cube.join(',');
        activeCubes.set(cubeKey, cube);
      }
    }
  }

  for (let i = 0; i < 6; i += 1) {
    const nextActiveCubes = new Map();
    const inactiveNeighborCubes = new Map();

    for (const [cubeKey, cube] of activeCubes) {
      const [x, y, z, w] = cube;

      let activeNeighborCount = 0;

      for (let dx = -1; dx <= 1; dx += 1) {
        for (let dy = -1; dy <= 1; dy += 1) {
          for (let dz = -1; dz <= 1; dz += 1) {
            for (let dw = -1; dw <= 1; dw += 1) {
              if (dx !== 0 || dy !== 0 || dz !== 0 || dw !== 0) {
                const neighborCubeKey = `${x + dx},${y + dy},${z + dz},${w + dw}`;

                if (activeCubes.has(neighborCubeKey)) {
                  activeNeighborCount += 1;
                } else if (!inactiveNeighborCubes.has(neighborCubeKey)) {
                  const neighborCube = [x + dx, y + dy, z + dz, w + dw];
                  inactiveNeighborCubes.set(neighborCubeKey, neighborCube);
                }
              }
            }
          }
        }
      }

      if (activeNeighborCount === 2 || activeNeighborCount === 3) {
        nextActiveCubes.set(cubeKey, cube);
      }
    }

    for (const [cubeKey, cube] of inactiveNeighborCubes) {
      const [x, y, z, w] = cube;

      let activeNeighborCount = 0;

      for (let dx = -1; dx <= 1; dx += 1) {
        for (let dy = -1; dy <= 1; dy += 1) {
          for (let dz = -1; dz <= 1; dz += 1) {
            for (let dw = -1; dw <= 1; dw += 1) {
              if (dx !== 0 || dy !== 0 || dz !== 0 || dw !== 0) {
                const neighborCubeKey = `${x + dx},${y + dy},${z + dz},${w + dw}`;

                if (activeCubes.has(neighborCubeKey)) {
                  activeNeighborCount += 1;
                }
              }
            }
          }
        }
      }

      if (activeNeighborCount === 3) {
        nextActiveCubes.set(cubeKey, cube);
      }
    }

    activeCubes = nextActiveCubes;
  }

  return activeCubes.size;
}
