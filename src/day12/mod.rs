use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec2<T>(pub [T; 2]);

impl<T, U> Add<Vec2<U>> for Vec2<T>
where
    T: Add<U>,
{
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Vec2<U>) -> Self::Output {
        let Vec2([x0, y0]) = self;
        let Vec2([x1, y1]) = rhs;
        Vec2([x0 + x1, y0 + y1])
    }
}

impl<T, U> AddAssign<Vec2<U>> for Vec2<T>
where
    T: Add<U, Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Vec2<U>) {
        *self = *self + rhs;
    }
}

impl<T, U> Mul<U> for Vec2<T>
where
    T: Mul<U>,
    U: Copy,
{
    type Output = Vec2<T::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        let Vec2([x, y]) = self;
        Vec2([x * rhs, y * rhs])
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Mat2<T>(pub [[T; 2]; 2]);

impl<T, U> Mul<Vec2<U>> for Mat2<T>
where
    T: Mul<U>,
    U: Copy,
    T::Output: Add,
{
    type Output = Vec2<<T::Output as Add>::Output>;

    fn mul(self, rhs: Vec2<U>) -> Self::Output {
        let Mat2([[a00, a01], [a10, a11]]) = self;
        let Vec2([x, y]) = rhs;
        Vec2([a00 * x + a01 * y, a10 * x + a11 * y])
    }
}

fn instructions(input: &str) -> impl Iterator<Item = (char, i32)> + '_ {
    input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
}

pub fn part1(input: &str) -> i32 {
    let rotations = [
        Mat2([[0, 1], [1, 0]]),   // 0 degress
        Mat2([[0, -1], [1, 0]]),  // 90 degress CCW
        Mat2([[-1, 0], [0, -1]]), // 180 degress
        Mat2([[0, 1], [-1, 0]]),  // 270 degress CCW
    ];

    let mut position = Vec2([0, 0]);
    let mut direction = Vec2([1, 0]);

    for (c, val) in instructions(input) {
        match c {
            'N' => position += Vec2([0, 1]) * val,
            'S' => position += Vec2([0, -1]) * val,
            'E' => position += Vec2([1, 0]) * val,
            'W' => position += Vec2([-1, 0]) * val,
            'L' => direction = rotations[((val / 90) % 4) as usize] * direction,
            'R' => direction = rotations[((-val / 90) % 4 + 4) as usize] * direction,
            'F' => position += direction * val,
            _ => {}
        }
    }

    let Vec2([x, y]) = position;
    x.abs() + y.abs()
}

pub fn part2(input: &str) -> i32 {
    let rotations = [
        Mat2([[0, 1], [1, 0]]),   // 0 degress
        Mat2([[0, -1], [1, 0]]),  // 90 degress CCW
        Mat2([[-1, 0], [0, -1]]), // 180 degress
        Mat2([[0, 1], [-1, 0]]),  // 270 degress CCW
    ];

    let mut position = Vec2([0, 0]);
    let mut direction = Vec2([10, 1]);

    for (c, val) in instructions(input) {
        match c {
            'N' => direction += Vec2([0, 1]) * val,
            'S' => direction += Vec2([0, -1]) * val,
            'E' => direction += Vec2([1, 0]) * val,
            'W' => direction += Vec2([-1, 0]) * val,
            'L' => direction = rotations[((val / 90) % 4) as usize] * direction,
            'R' => direction = rotations[((-val / 90) % 4 + 4) as usize] * direction,
            'F' => position += direction * val,
            _ => {}
        }
    }

    let Vec2([x, y]) = position;
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("test_input.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 25);
        assert_eq!(part1(INPUT), 1106);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 286);
        assert_eq!(part2(INPUT), 107_281);
    }
}
