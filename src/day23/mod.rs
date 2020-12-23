type Cup = usize;

fn parse_cups(input: &str) -> impl Iterator<Item = Cup> + '_ {
    input.chars().map(|c| (c.to_digit(10).unwrap() - 1) as Cup)
}

struct Game {
    pub current_cup: Cup,
    pub next_cups: Box<[Cup]>,
}

impl Game {
    pub fn new(initial_cups: &[Cup], cups_count: usize) -> Self {
        let mut cups = Vec::with_capacity(cups_count);

        cups.extend_from_slice(initial_cups);
        cups.extend(initial_cups.len()..cups_count);

        let mut next_cups = vec![0; cups_count];

        for i in 0..cups.len() {
            let cup = cups[i];
            let next_cup = cups[(i + 1) % cups.len()];
            next_cups[cup] = next_cup;
        }

        Self {
            current_cup: initial_cups[0],
            next_cups: next_cups.into(),
        }
    }

    #[inline]
    pub fn make_move(&mut self) {
        let picked_cup_0 = self.next_cups[self.current_cup];
        let picked_cup_1 = self.next_cups[picked_cup_0];
        let picked_cup_2 = self.next_cups[picked_cup_1];

        let next_cup = self.next_cups[picked_cup_2];
        self.next_cups[self.current_cup] = next_cup;

        let destination_cup = {
            let len = self.next_cups.len();
            let mut d = self.current_cup;
            loop {
                d = (d + len - 1) % len;
                if d != picked_cup_0 && d != picked_cup_1 && d != picked_cup_2 {
                    break d;
                }
            }
        };

        self.next_cups[picked_cup_2] = self.next_cups[destination_cup];
        self.next_cups[destination_cup] = picked_cup_0;

        self.current_cup = next_cup;
    }
}

pub fn part1(input: &str, moves: usize) -> String {
    let initial_cups: Vec<Cup> = parse_cups(input).collect();

    let mut game = Game::new(&initial_cups, initial_cups.len());

    for _ in 0..moves {
        game.make_move();
    }

    use std::fmt::Write;

    let mut stringified = String::with_capacity(game.next_cups.len() - 1);
    let mut current_cup = game.next_cups[0];

    for _ in 0..game.next_cups.len() - 1 {
        write!(&mut stringified, "{}", current_cup + 1).unwrap();
        current_cup = game.next_cups[current_cup];
    }

    stringified
}

pub fn part2(input: &str) -> u64 {
    let initial_cups: Vec<Cup> = parse_cups(input).collect();

    let mut game = Game::new(&initial_cups, 1_000_000);

    for _ in 0..10_000_000 {
        game.make_move();
    }

    let cup1 = game.next_cups[0];
    let cup2 = game.next_cups[cup1];

    ((cup1 as u64) + 1) * ((cup2 as u64) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("389125467", 10), "92658374");
        assert_eq!(part1("389125467", 100), "67384529");
        assert_eq!(part1("315679824", 100), "72496583");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("389125467"), 149_245_887_792);
        assert_eq!(part2("315679824"), 41_785_843_847);
    }
}
