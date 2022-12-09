use ::lending_iterator::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Clone, Debug, PartialEq, EnumString)]
enum Direction {
    R,
    L,
    U,
    D,
}

fn moves(input: &str) -> Vec<Direction> {
    input
        .split("\n")
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .flat_map(|vec| {
            let dir = Direction::from_str(vec[0]).unwrap();
            let repeat = vec[1].parse::<usize>().unwrap();
            vec![dir; repeat]
        })
        .collect::<Vec<Direction>>()
}

pub fn part1(input: &str) -> u16 {
    let mut h_pos: (i16, i16) = (0, 0);
    let mut t_pos: (i16, i16) = (0, 0);
    let mut t_visited = HashSet::from([t_pos]);
    let move_set = moves(&input);

    t_visited.insert(t_pos);

    for h_move in move_set.iter() {
        match h_move {
            Direction::R => {
                h_pos.0 += 1;
            }
            Direction::L => {
                h_pos.0 -= 1;
            }
            Direction::U => {
                h_pos.1 += 1;
            }
            Direction::D => {
                h_pos.1 -= 1;
            }
        }

        let x_range = (h_pos.0 - 1)..=(h_pos.0 + 1);
        let y_range = (h_pos.1 - 1)..=(h_pos.1 + 1);

        let is_tail_connected = x_range
            .cartesian_product(y_range)
            .any(|coords| coords == t_pos);

        if !is_tail_connected {
            let mut new_t_pos = h_pos.clone();

            match h_move {
                Direction::R => {
                    new_t_pos.0 -= 1;
                }
                Direction::L => {
                    new_t_pos.0 += 1;
                }
                Direction::U => {
                    new_t_pos.1 -= 1;
                }
                Direction::D => {
                    new_t_pos.1 += 1;
                }
            }

            t_pos = new_t_pos;
            t_visited.insert(t_pos);
        }

        // dbg!(h_pos);
        // dbg!(t_pos);
        // dbg!();
    }

    t_visited.len().try_into().unwrap()
}

pub fn part2(input: &str) -> usize {
    let move_set = moves(&input);
    let mut rope = [(0, 0); 10];
    let mut tail_positions =
        HashSet::from([*rope.last().unwrap()]);

    for head_move in move_set.iter() {
        match head_move {
            Direction::L => {
                rope[0].0 -= 1;
            }
            Direction::R => {
                rope[0].0 += 1;
            }
            Direction::U => {
                rope[0].1 += 1;
            }
            Direction::D => {
                rope[0].1 -= 1;
            }
        }

        let mut rope_windows = rope.windows_mut::<2>();
        while let Some([ref mut head, ref mut tail]) =
            rope_windows.next()
        {
            // println!("{:?}{:?}", head, tail);
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            let tail_is_connected = x_range
                .cartesian_product(y_range)
                .any(|tuple| tuple == *tail);

            if !tail_is_connected {
                // println!("{last_head_move:?}");
                // move_tail
                // let mut new_tail = head.clone();
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else {
                    // diagonal
                    // let head_cross_positions = [
                    //     (head.0 - 1, head.1),
                    //     (head.0 + 1, head.1),
                    //     (head.0, head.1 - 1),
                    //     (head.0, head.1 + 1),
                    // ];
                    let x_range =
                        (head.0 - 1)..=(head.0 + 1);
                    let y_range =
                        (head.1 - 1)..=(head.1 + 1);

                    let head_3x3 = x_range
                        .cartesian_product(y_range)
                        .collect::<Vec<_>>();

                    let x_range =
                        (tail.0 - 1)..=(tail.0 + 1);
                    let y_range =
                        (tail.1 - 1)..=(tail.1 + 1);

                    let maybe_new_tail: Vec<(i32, i32)> =
                        x_range
                            .cartesian_product(y_range)
                            .filter(|tuple| {
                                head_3x3.contains(tuple)
                            })
                            .collect();
                    match maybe_new_tail.len() {
                        2 => {
                            let new_head_cross_positions = [
                                (head.0 - 1, head.1),
                                (head.0 + 1, head.1),
                                (head.0, head.1 - 1),
                                (head.0, head.1 + 1),
                            ];
                            let next = maybe_new_tail
                                .iter()
                                .find(|tuple| {
                                    new_head_cross_positions
                                        .contains(tuple)
                                })
                                .unwrap();
                            *tail = *next;
                        }
                        1 => {
                            *tail = maybe_new_tail[0];
                        }
                        _ => {
                            panic!("unknown tail length");
                        }
                    };
                    // *tail = new_tail;
                }
            }
        }

        tail_positions.insert(*rope.last().unwrap());
    }
    tail_positions.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT_2);
        assert_eq!(result, 36);
    }
}
