use std::mem;

fn get_number(initial_numbers: &[usize], index: usize) -> usize {
    if let Some(value) = initial_numbers.get(index) {
        return *value;
    }

    let mut prev_turns = vec![None; index + 1];

    for (i, val) in initial_numbers.iter().enumerate() {
        prev_turns[*val] = Some(i);
    }

    let mut turn = initial_numbers.len() - 1;
    let mut number = initial_numbers[turn];

    while turn < index {
        let prev_turn = mem::replace(&mut prev_turns[number], Some(turn));
        number = prev_turn.map_or(0, |prev_turn| turn - prev_turn);
        turn += 1;
    }

    number
}

pub fn part1(input: &str) -> usize {
    let numbers: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    get_number(&numbers, 2020 - 1)
}

pub fn part2(input: &str) -> usize {
    let numbers: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    get_number(&numbers, 30_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        assert_eq!(part1("0,3,6"), 436);
        assert_eq!(part1("5,1,9,18,13,8,0"), 376);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("5,1,9,18,13,8,0"), 323_780);
    }
}
