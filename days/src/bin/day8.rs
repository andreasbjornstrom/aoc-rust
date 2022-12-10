use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let data = "30373
25512
65332
33549
35390";
    //let day = 0;
    let data = &aoc::get_aoc_file(8);
    let sum_part1: usize = parse_things_part1(data);
    let sum_part2: usize = parse_things_part2(data);
    println!("part 1 {:?}", sum_part1);
    println!("part 2 {:?}", sum_part2)
}

fn parse_things_part1(data: &str) -> usize {
    let mut visibile: HashSet<(usize, usize)> = HashSet::new();

    let rows = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    // from left
    for (x, row) in rows.iter().enumerate() {
        let mut highest_so_far = -1;
        for (y, tree) in row.iter().enumerate() {
            if tree > &highest_so_far {
                visibile.insert((x, y));
                highest_so_far = *tree;
                println!("adding: {:?}, with value: {highest_so_far}", (x, y));
            }
        }
    }
    // from right
    println!("{:?}", visibile.len());
    for (x, row) in rows.iter().enumerate() {
        let mut highest_so_far = -1;
        for (y, tree) in row.iter().enumerate().rev() {
            if tree > &highest_so_far {
                visibile.insert((x, y));
                highest_so_far = *tree;
                println!("adding: {:?}, with value: {highest_so_far}", (x, y));
            }
        }
    }
    println!("{:?}", visibile);
    println!("{:?}", visibile.len());
    // from top
    for y in 0..rows[0].len() {
        let mut highest_so_far = -1;
        for x in 0..rows.len() {
            let tree = rows[x][y];
            if tree > highest_so_far {
                visibile.insert((x, y));
                highest_so_far = tree;
                println!("adding: {:?}, with value: {highest_so_far}", (x, y));
            }
        }
    }
    println!("{:?}", visibile);
    println!("{:?}", visibile.len());
    // from bottom
    for y in 0..rows[0].len() {
        let mut highest_so_far = -1;
        for x in 0..rows.len() {
            let x = rows.len() - x - 1;
            let tree = rows[x][y];
            if tree > highest_so_far {
                visibile.insert((x, y));
                highest_so_far = tree;
                println!("adding: {:?}, with value: {highest_so_far}", (x, y));
            }
        }
    }

    println!("{:?}", visibile);
    println!("{:?}", visibile.len());
    return visibile.len();
}

fn parse_things_part2(data: &str) -> usize {
    let mut visibile: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    let mut max_sum = 0;
    let rows = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    for (spot_tree_x, row) in rows.iter().enumerate() {
        for (spot_tree_y, spot_tree) in row.iter().enumerate() {
            println!("Processing {spot_tree_x}, {spot_tree_y}");
            visibile.clear();
            // from left
            'outer1: for (x, row) in rows.iter().enumerate() {
                let mut highest_so_far = -1;
                for (y, tree) in row.iter().enumerate() {
                    if spot_tree_x != x || y < spot_tree_y {
                        continue;
                    }
                    if highest_so_far >= *spot_tree {
                        break 'outer1;
                    }
                    if spot_tree_x == x && spot_tree_y == y {
                        continue;
                    }
                    visibile.insert((x, y));
                    highest_so_far = *tree;
                    println!("adding: {:?}, with value: {highest_so_far}", (x, y));
                }
            }
            if visibile.is_empty() {
                continue;
            }
            sum = visibile.len();
            println!("sum: {sum}");
            println!("{:?}", visibile.len());
            visibile.clear();
            // from right
            'outer2: for (x, row) in rows.iter().enumerate() {
                let mut highest_so_far = &-1;
                for (y, tree) in row.iter().enumerate().rev() {
                    if spot_tree_x != x || y > spot_tree_y {
                        continue;
                    }
                    if highest_so_far >= spot_tree {
                        break 'outer2;
                    }
                    if spot_tree_x == x && spot_tree_y == y {
                        continue;
                    }
                    visibile.insert((x, y));
                    highest_so_far = tree;
                    println!("adding: {:?}, with value: {highest_so_far}", (x, y));
                }
            }
            println!("{:?}", visibile);
            println!("{:?}", visibile.len());
            if visibile.is_empty() {
                continue;
            }
            sum *= visibile.len();
            println!("sum: {sum}");
            visibile.clear();
            // from top
            'outer3: for y in 0..rows[0].len() {
                let mut highest_so_far = -1;
                for x in 0..rows.len() {
                    if spot_tree_x > x || spot_tree_y != y {
                        continue;
                    }
                    if highest_so_far >= *spot_tree {
                        break 'outer3;
                    }
                    if spot_tree_x == x && spot_tree_y == y {
                        continue;
                    }
                    let tree = rows[x][y];
                    visibile.insert((x, y));
                    highest_so_far = tree;
                    println!("adding: {:?}, with value: {highest_so_far}", (x, y));
                }
            }
            println!("{:?}", visibile);
            println!("{:?}", visibile.len());
            // from bottom
            if visibile.is_empty() {
                continue;
            }
            sum *= visibile.len();
            println!("sum: {sum}");
            visibile.clear();
            'outer4: for y in 0..rows[0].len() {
                let mut highest_so_far = -1;
                for x in 0..rows.len() {
                    let x = rows.len() - x - 1;
                    if spot_tree_x < x || spot_tree_y != y {
                        continue;
                    }
                    if highest_so_far >= *spot_tree {
                        break 'outer4;
                    }
                    if spot_tree_x == x && spot_tree_y == y {
                        continue;
                    }
                    let tree = rows[x][y];
                    visibile.insert((x, y));
                    highest_so_far = tree;
                    println!("adding: {:?}, with value: {highest_so_far}", (x, y));
                }
            }
            if visibile.is_empty() {
                continue;
            }
            sum *= visibile.len();
            visibile.clear();
            println!("tot sum: {sum}");
            if sum > max_sum {
                max_sum = sum
            }
            println!("max sum: {max_sum}");
            sum = 0
        }
    }
    println!("{:?}", visibile);
    println!("{:?}", visibile.len());

    return max_sum;
}
