use std::{collections::HashMap, writeln};

const TILE_SIZE: usize = 10;

type TileId = u16;
type Tile = [[bool; TILE_SIZE]; TILE_SIZE];

type Position = [i16; 2];

type RemainingTiles<'a> = HashMap<TileId, &'a Tile>;
type TileArrangement<'a> = HashMap<Position, (TileId, TileView<'a>)>;

#[derive(Debug, Copy, Clone)]
enum Rotation {
    None,
    Right,
    Half,
    Left,
}

#[derive(Debug, Clone)]
struct TileView<'a> {
    tile: &'a Tile,
    transposed: bool,
    rotation: Rotation,
}

impl<'a> std::ops::Index<[usize; 2]> for TileView<'a> {
    type Output = bool;

    fn index(&self, [mut x, mut y]: [usize; 2]) -> &Self::Output {
        if self.transposed {
            std::mem::swap(&mut x, &mut y);
        }

        match self.rotation {
            Rotation::None => {}
            Rotation::Right => {
                let prev_x = x;
                x = TILE_SIZE - 1 - y;
                y = prev_x;
            }
            Rotation::Half => {
                x = TILE_SIZE - 1 - x;
                y = TILE_SIZE - 1 - y;
            }
            Rotation::Left => {
                let prev_x = x;
                x = y;
                y = TILE_SIZE - 1 - prev_x;
            }
        }

        &self.tile[y][x]
    }
}

impl<'a> std::fmt::Display for TileView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}, {:?}", self.transposed, self.rotation);
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                write!(f, "{}", if self[[y, x]] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_tiles(input: &str) -> impl Iterator<Item = (TileId, Tile)> + '_ {
    input.split("\n\n").map(|chunk| {
        let mut lines = chunk.lines();

        let first_line = lines.next().unwrap();
        let id = first_line
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse()
            .unwrap();

        let mut data = [[false; TILE_SIZE]; TILE_SIZE];
        for (y, line) in lines.enumerate() {
            let y_data = &mut data[y];
            for (x, c) in line.chars().enumerate() {
                y_data[x] = match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid char: {}", c),
                };
            }
        }

        (id, data)
    })
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Top,
    Direction::Bottom,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    const fn opposite(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Left => Self::Right,
            Self::Bottom => Self::Top,
            Self::Right => Self::Left,
        }
    }
}

impl TileView<'_> {
    fn edge(&self, dir: Direction) -> [bool; TILE_SIZE] {
        let mut edge = [false; TILE_SIZE];

        match dir {
            Direction::Top => {
                for i in 0..TILE_SIZE {
                    edge[i] = self[[0, i]];
                }
            }
            Direction::Left => {
                for i in 0..TILE_SIZE {
                    edge[i] = self[[i, 0]];
                }
            }
            Direction::Bottom => {
                for i in 0..TILE_SIZE {
                    edge[i] = self[[TILE_SIZE - 1, i]];
                }
            }
            Direction::Right => {
                for i in 0..TILE_SIZE {
                    edge[i] = self[[i, TILE_SIZE - 1]];
                }
            }
        };

        edge
    }
}

const ROTATIONS: [Rotation; 4] = [
    Rotation::None,
    Rotation::Right,
    Rotation::Half,
    Rotation::Left,
];

const TRUE_OR_FALSE: [bool; 2] = [false, true];

fn tile_views(tile: &Tile) -> impl Iterator<Item = TileView<'_>> {
    TRUE_OR_FALSE.iter().copied().flat_map(move |transposed| {
        ROTATIONS.iter().copied().map(move |rotation| TileView {
            tile,
            transposed,
            rotation,
        })
    })
}

fn neighbors(pos: Position) -> impl Iterator<Item = (Direction, Position)> {
    let [x, y] = pos;
    DIRECTIONS.iter().copied().map(move |dir| {
        (
            dir,
            match dir {
                Direction::Top => [x, y - 1],
                Direction::Bottom => [x, y + 1],
                Direction::Left => [x - 1, y],
                Direction::Right => [x + 1, y],
            },
        )
    })
}

fn bounding_box<'a>(positions: impl IntoIterator<Item = &'a Position>) -> Option<[Position; 2]> {
    positions.into_iter().fold(
        Option::<[Position; 2]>::None,
        |result, &[x, y]: &'a Position| {
            if let Some([[min_x, min_y], [max_x, max_y]]) = result {
                Some([[min_x.min(x), min_y.min(y)], [max_x.max(x), max_y.max(y)]])
            } else {
                Some([[x, y], [x, y]])
            }
        },
    )
}

fn find_tile_arrangement_helper<'a>(
    remaining_tiles: RemainingTiles<'a>,
    tile_arrangement: TileArrangement<'a>,
) -> Option<TileArrangement<'a>> {
    if remaining_tiles.is_empty() {
        Some(tile_arrangement)
    } else {
        remaining_tiles.iter().find_map(|(&tile_id, &tile)| {
            tile_arrangement
                .keys()
                .copied()
                .flat_map(neighbors)
                .filter_map(|(_, pos)| {
                    if tile_arrangement.contains_key(&pos) {
                        None
                    } else {
                        Some(pos)
                    }
                })
                .find_map(|pos| {
                    tile_views(tile).find_map(|tile_view| {
                        let valid = neighbors(pos)
                            .filter_map(|(dir, neighbor_pos)| {
                                Some((dir, tile_arrangement.get(&neighbor_pos)?))
                            })
                            .all(|(dir, (_, neighbor_tile_view))| {
                                neighbor_tile_view.edge(dir.opposite()) == tile_view.edge(dir)
                            });

                        if valid {
                            let mut next_remaining_tiles = remaining_tiles.clone();
                            next_remaining_tiles.remove(&tile_id);

                            let mut next_tile_arrangement = tile_arrangement.clone();
                            next_tile_arrangement.insert(pos, (tile_id, tile_view));

                            find_tile_arrangement_helper(
                                next_remaining_tiles,
                                next_tile_arrangement,
                            )
                        } else {
                            None
                        }
                    })
                })
        })
    }
}

fn find_tile_arrangement<'a>(
    tiles: impl IntoIterator<Item = &'a (TileId, Tile)>,
) -> Option<TileArrangement<'a>> {
    let mut tile_arrangement = TileArrangement::new();
    let mut remaining_tiles: RemainingTiles = tiles
        .into_iter()
        .map(|(tile_id, tile)| (*tile_id, tile))
        .collect();

    let (&tile_id, _) = remaining_tiles.iter().next()?;
    let tile = remaining_tiles.remove(&tile_id)?;

    tile_arrangement.insert(
        [0, 0],
        (
            tile_id,
            TileView {
                tile,
                transposed: false,
                rotation: Rotation::Right,
            },
        ),
    );

    find_tile_arrangement_helper(remaining_tiles, tile_arrangement)
}

pub fn part1(input: &str) -> u64 {
    let tiles: Vec<(TileId, Tile)> = parse_tiles(input).collect();

    let tile_arrangement = find_tile_arrangement(&tiles).expect("No tile arrangement found!");

    let [[min_x, min_y], [max_x, max_y]] = bounding_box(tile_arrangement.keys()).unwrap();

    let corners = [
        [min_x, min_y],
        [min_x, max_y],
        [max_x, min_y],
        [max_x, max_y],
    ];

    corners
        .iter()
        .map(|corner| tile_arrangement.get(corner).expect("Not a square!"))
        .map(|(tile_id, _)| *tile_id as u64)
        .product()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Pixel {
    Empty,
    Filled,
    SeaMonster,
}

struct MapView<'a> {
    map: &'a mut [Pixel],
    size: usize,
    transposed: bool,
    rotation: Rotation,
}

impl MapView<'_> {
    fn index(&self, mut x: usize, mut y: usize) -> usize {
        if self.transposed {
            std::mem::swap(&mut x, &mut y);
        }

        match self.rotation {
            Rotation::None => {}
            Rotation::Right => {
                let prev_x = x;
                x = self.size - 1 - y;
                y = prev_x;
            }

            Rotation::Half => {
                x = self.size - 1 - x;
                y = self.size - 1 - y;
            }
            Rotation::Left => {
                let prev_x = x;
                x = y;
                y = self.size - 1 - prev_x;
            }
        }

        y * self.size + x
    }
}

impl<'a> std::ops::Index<[usize; 2]> for MapView<'a> {
    type Output = Pixel;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        &self.map[self.index(x, y)]
    }
}

impl<'a> std::ops::IndexMut<[usize; 2]> for MapView<'a> {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        &mut self.map[self.index(x, y)]
    }
}

impl<'a> std::fmt::Display for MapView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}, {:?}", self.transposed, self.rotation)?;
        for y in 0..self.size {
            writeln!(f)?;
            for x in 0..self.size {
                let pixel = self[[x, y]];
                write!(
                    f,
                    "{}",
                    match pixel {
                        Pixel::Empty => '.',
                        Pixel::Filled => '#',
                        Pixel::SeaMonster => 'O',
                    }
                )?;
            }
        }
        Ok(())
    }
}

const SEA_MONSTER_WIDTH: usize = 20;
const SEA_MONSTER_HEIGHT: usize = 3;

const SEA_MONSTER: [&[u8; SEA_MONSTER_WIDTH]; SEA_MONSTER_HEIGHT] = [
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
];

pub fn part2(input: &str) -> usize {
    let tiles: Vec<(TileId, Tile)> = parse_tiles(input).collect();

    let tile_arrangement = find_tile_arrangement(&tiles).expect("No tile arrangement found!");

    let [[min_x, min_y], [max_x, max_y]] = bounding_box(tile_arrangement.keys()).unwrap();

    // for y in min_y..=max_y {
    //     println!();
    //     for x in min_x..=max_x {
    //         let (tile_id, _) = tile_arrangement.get(&[x, y]).unwrap();
    //         print!("{} ", tile_id);
    //     }
    // }

    let tiles_size = {
        let ncols = 1 + max_x - min_x;
        let nrows = 1 + max_y - min_y;
        assert_eq!(ncols, nrows);
        ncols as usize
    };

    let map_size = (TILE_SIZE - 2) * tiles_size;

    let mut map: Vec<Pixel> = Vec::with_capacity(map_size * map_size);

    for y in 0..map_size {
        for x in 0..map_size {
            let (_, tile_view) = tile_arrangement
                .get(&[
                    min_x + (x / (TILE_SIZE - 2)) as i16,
                    min_y + (y / (TILE_SIZE - 2)) as i16,
                ])
                .unwrap();

            let filled = tile_view[[1 + y % (TILE_SIZE - 2), 1 + x % (TILE_SIZE - 2)]];

            map.push(if filled { Pixel::Filled } else { Pixel::Empty });
        }
    }

    for &transposed in TRUE_OR_FALSE.iter() {
        for &rotation in ROTATIONS.iter() {
            let mut map_view = MapView {
                map: &mut map,
                size: map_size,
                transposed,
                rotation,
            };

            for y0 in 0..(map_size - SEA_MONSTER_HEIGHT) {
                'check_sea_monster: for x0 in 0..(map_size - SEA_MONSTER_WIDTH) {
                    for y in 0..SEA_MONSTER_HEIGHT {
                        for x in 0..SEA_MONSTER_WIDTH {
                            if SEA_MONSTER[y][x] == b'#'
                                && map_view[[y0 + y, x0 + x]] == Pixel::Empty
                            {
                                continue 'check_sea_monster;
                            }
                        }
                    }

                    for y in 0..SEA_MONSTER_HEIGHT {
                        for x in 0..SEA_MONSTER_WIDTH {
                            if SEA_MONSTER[y][x] == b'#' {
                                map_view[[y0 + y, x0 + x]] = Pixel::SeaMonster;
                            }
                        }
                    }
                }
            }
        }
    }

    map.iter().filter(|pixel| **pixel == Pixel::Filled).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 20_899_048_083_289);
        assert_eq!(part1(INPUT), 29_125_888_761_511);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 273);
        assert_eq!(part2(INPUT), 2219);
    }
}
