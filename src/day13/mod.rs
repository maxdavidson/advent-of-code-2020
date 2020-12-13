struct Notes {
    departure_timestamp: u64,
    bus_ids: Box<[Option<u64>]>,
}

impl<'a> From<&'a str> for Notes {
    fn from(s: &'a str) -> Self {
        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let second_line = lines.next().unwrap();
        Self {
            departure_timestamp: first_line.parse().unwrap(),
            bus_ids: second_line.split(',').map(|s| s.parse().ok()).collect(),
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let notes = Notes::from(input);

    for timestamp in notes.departure_timestamp.. {
        for bus_id in notes.bus_ids.iter().flatten().copied() {
            if timestamp % bus_id == 0 {
                return (timestamp - notes.departure_timestamp) * bus_id;
            }
        }
    }

    panic!("No solution found");
}

pub fn part2(input: &str) -> u64 {
    let notes = Notes::from(input);

    let mut timestamp = 0;
    let mut stride = 1;

    for (maybe_bus_id, offset) in notes.bus_ids.iter().copied().zip(0u64..) {
        if let Some(bus_id) = maybe_bus_id {
            while (timestamp + offset) % bus_id != 0 {
                timestamp += stride
            }
            stride *= bus_id
        }
    }

    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1("939\n7,13,x,x,59,x,31,19"), 295);
        assert_eq!(part1(INPUT), 222);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("0\n17,x,13,19"), 3417);
        assert_eq!(part2("0\n67,7,59,61"), 754_018);
        assert_eq!(part2("0\n67,x,7,59,61"), 779_210);
        assert_eq!(part2("0\n67,7,x,59,61"), 1_261_476);
        assert_eq!(part2("0\n1789,37,47,1889"), 1_202_161_486);
        assert_eq!(part2(INPUT), 408_270_049_879_073);
    }
}
