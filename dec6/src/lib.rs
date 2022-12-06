pub fn part1(input: &str) -> usize {
    let mut packet_marker = 0;

    for i in 1..input.len() {
        let slice = &input[(i - 1)..(i + 3)];
        let mut slice_has_duplicates: Vec<bool> = Vec::new();

        for (index, c) in slice.chars().enumerate() {
            if !slice[(index + 1)..].contains(c) {
                slice_has_duplicates.push(false);
            } else {
                slice_has_duplicates.push(true);
                break;
            }
        }

        if !slice_has_duplicates.contains(&true) {
            packet_marker = i + 3;
            break;
        }
    };

    packet_marker
}

pub fn part2(input: &str) -> usize {
    let mut message_marker = 0;

    for i in 1..input.len() {
        let slice = &input[(i - 1)..(i + 13)];
        let mut slice_has_duplicates: Vec<bool> = Vec::new();

        for (index, c) in slice.chars().enumerate() {
            if !slice[(index + 1)..].contains(c) {
                slice_has_duplicates.push(false);
            } else {
                slice_has_duplicates.push(true);
                break;
            }
        }

        if !slice_has_duplicates.contains(&true) {
            message_marker = i + 13;
            break;
        }
    };

    message_marker
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 19);
    }
}
