#[derive(Debug, Copy, Clone)]
struct Tree {
    h: u16,
    visibility: u64,
    scenic_score: u64,
}

pub fn part1(input: &str) -> usize {
    let tree_grid: Vec<Vec<u16>> = input
        .split("\n")
        .map(|line| {
            line.split("")
                .filter_map(|t| t.parse::<u16>().ok())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();
    let mut tree_house_candidates: Vec<Tree> = Vec::new();

    for i in 1..(tree_grid.len() - 1) {
        for j in 1..(tree_grid[i].len() - 1) {
            let mut tree = Tree {
                h: tree_grid[i][j],
                visibility: 0,
                scenic_score: 0,
            };
            let mut visible_left: u64 = 0;
            let mut visible_right: u64 = 0;
            let mut visible_top: u64 = 0;
            let mut visible_bottom: u64 = 0;

            for k in tree_grid[i][0..j].iter().rev() {
                if k >= &tree.h {
                    visible_left += 1;
                    break;
                } else {
                    visible_left += 1;
                }
            }

            for k in tree_grid[i][(j + 1)..tree_grid[i].len()].iter() {
                if k >= &tree.h {
                    visible_right += 1;
                    break;
                } else {
                    visible_right += 1;
                }
            }

            let curr_col = tree_grid.iter().map(|x| x[j]).collect::<Vec<u16>>();

            for k in curr_col[0..i].iter().rev() {
                if k >= &tree.h {
                    visible_top += 1;
                    break;
                } else {
                    visible_top += 1;
                }
            }

            for k in curr_col[(i + 1)..curr_col.len()].iter() {
                if k >= &tree.h {
                    visible_bottom += 1;
                    break;
                } else {
                    visible_bottom += 1;
                }
            }

            tree.visibility = visible_left + visible_right + visible_top + visible_bottom;
            tree_house_candidates.push(tree);
        }
    }

    let visible_trees = &tree_house_candidates
        .iter()
        .enumerate()
        .filter(|&(_, &t)| t.visibility > 4)
        .map(|(_, &t)| t)
        .collect::<Vec<Tree>>();

    let total_visible_trees =
        visible_trees.len() + (2 * tree_grid.len()) + (2 * (tree_grid[0].len() - 2));
    total_visible_trees
}

pub fn part2(input: &str) -> u64 {
    let tree_grid: Vec<Vec<u16>> = input
        .split("\n")
        .map(|line| {
            line.split("")
                .filter_map(|t| t.parse::<u16>().ok())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();
    let mut tree_house_candidates: Vec<Tree> = Vec::new();

    for i in 1..(tree_grid.len() - 1) {
        for j in 1..(tree_grid[i].len() - 1) {
            let mut tree = Tree {
                h: tree_grid[i][j],
                visibility: 0,
                scenic_score: 0,
            };
            let mut visible_left: u64 = 0;
            let mut visible_right: u64 = 0;
            let mut visible_top: u64 = 0;
            let mut visible_bottom: u64 = 0;

            for k in tree_grid[i][0..j].iter().rev() {
                if k >= &tree.h {
                    visible_left += 1;
                    break;
                } else {
                    visible_left += 1;
                }
            }

            for k in tree_grid[i][(j + 1)..tree_grid[i].len()].iter() {
                if k >= &tree.h {
                    visible_right += 1;
                    break;
                } else {
                    visible_right += 1;
                }
            }

            let curr_col = tree_grid.iter().map(|x| x[j]).collect::<Vec<u16>>();

            for k in curr_col[0..i].iter().rev() {
                if k >= &tree.h {
                    visible_top += 1;
                    break;
                } else {
                    visible_top += 1;
                }
            }

            for k in curr_col[(i + 1)..curr_col.len()].iter() {
                if k >= &tree.h {
                    visible_bottom += 1;
                    break;
                } else {
                    visible_bottom += 1;
                }
            }

            tree.visibility = visible_left + visible_right + visible_top + visible_bottom;
            tree.scenic_score = visible_top * visible_bottom * visible_right * visible_left;
            tree_house_candidates.push(tree);
        }
    }

    tree_house_candidates
        .iter()
        .map(|t| t.scenic_score)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 8);
    }
}
