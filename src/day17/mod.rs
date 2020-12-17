use std::collections::HashSet;

type Int = i8;

fn parse_input(input: &str) -> impl Iterator<Item = (Int, Int)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(move |(x, _)| (x as Int, y as Int))
    })
}

pub fn part1(input: &str) -> usize {
    let mut active_cubes: HashSet<_> = parse_input(input).map(|(x, y)| (x, y, 0)).collect();

    for _ in 0..6 {
        let mut next_active_cubes = HashSet::new();
        let mut inactive_neighbor_cubes = HashSet::new();

        for cube in active_cubes.iter().copied() {
            let mut active_neighbor_count = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if (dx, dy, dz) != (0, 0, 0) {
                            let (x, y, z) = cube;
                            let neighbor_cube = (x + dx, y + dy, z + dz);

                            if active_cubes.contains(&neighbor_cube) {
                                active_neighbor_count += 1;
                            } else {
                                inactive_neighbor_cubes.insert(neighbor_cube);
                            }
                        }
                    }
                }
            }

            if active_neighbor_count == 2 || active_neighbor_count == 3 {
                next_active_cubes.insert(cube);
            }
        }

        for cube in inactive_neighbor_cubes {
            let mut active_neighbor_count = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if (dx, dy, dz) != (0, 0, 0) {
                            let (x, y, z) = cube;
                            let neighbor_cube = (x + dx, y + dy, z + dz);

                            if active_cubes.contains(&neighbor_cube) {
                                active_neighbor_count += 1;
                            }
                        }
                    }
                }
            }

            if active_neighbor_count == 3 {
                next_active_cubes.insert(cube);
            }
        }

        active_cubes = next_active_cubes;
    }

    active_cubes.len()
}

pub fn part2(input: &str) -> usize {
    let mut active_cubes: HashSet<_> = parse_input(input).map(|(x, y)| (x, y, 0, 0)).collect();

    for _ in 0..6 {
        let mut next_active_cubes = HashSet::new();
        let mut inactive_neighbor_cubes = HashSet::new();

        for cube in active_cubes.iter().copied() {
            let mut active_neighbor_count = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if (dx, dy, dz, dw) != (0, 0, 0, 0) {
                                let (x, y, z, w) = cube;
                                let neighbor_cube = (x + dx, y + dy, z + dz, w + dw);

                                if active_cubes.contains(&neighbor_cube) {
                                    active_neighbor_count += 1;
                                } else {
                                    inactive_neighbor_cubes.insert(neighbor_cube);
                                }
                            }
                        }
                    }
                }
            }

            if active_neighbor_count == 2 || active_neighbor_count == 3 {
                next_active_cubes.insert(cube);
            }
        }

        for cube in inactive_neighbor_cubes {
            let mut active_neighbor_count = 0;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if (dx, dy, dz, dw) != (0, 0, 0, 0) {
                                let (x, y, z, w) = cube;
                                let neighbor_cube = (x + dx, y + dy, z + dz, w + dw);

                                if active_cubes.contains(&neighbor_cube) {
                                    active_neighbor_count += 1;
                                }
                            }
                        }
                    }
                }
            }

            if active_neighbor_count == 3 {
                next_active_cubes.insert(cube);
            }
        }

        active_cubes = next_active_cubes;
    }

    active_cubes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 112);
        assert_eq!(part1(INPUT), 240);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 848);
        assert_eq!(part2(INPUT), 1180);
    }
}
