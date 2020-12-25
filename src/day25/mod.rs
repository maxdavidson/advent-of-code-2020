pub fn part1(door_public_key: u64, card_public_key: u64) -> u64 {
    let mut loop_size = 0;
    let mut public_key = 1;

    while public_key != door_public_key && public_key != card_public_key {
        public_key = (public_key * 7) % 20201227;
        loop_size += 1;
    }

    let subject_number = if public_key == door_public_key {
        card_public_key
    } else {
        door_public_key
    };

    let mut encryption_key = 1;

    while loop_size != 0 {
        encryption_key = (encryption_key * subject_number) % 20201227;
        loop_size -= 1;
    }

    encryption_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(5764801, 17807724), 14897079);
        assert_eq!(part1(1614360, 7734663), 5414549);
    }
}
