use std::collections::HashSet;
use std::ops::Index;
use aoc::get_aoc_file;

fn part_1() {
    let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw.";
    let data = get_aoc_file(3);
    let result: usize = data.split("\n")
        .map(get_duplicates)
        .sum();

// split by len.
// search if found in other vec?
// get point if found?
// sum..
    println!("{:?}", result);
}

fn main() {
    let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw.";
    let data = get_aoc_file(3);
    let result: Vec<&str> = data.split("\n").collect();
    let mut sum = 0;
    for i in 1..result.len() +1 {
        if i % 3 == 0 {
            println!("comparing.. from row nr {:?}", i);
            sum += compare_groups_part2(result[i - 3], result[i - 2], result[i-1]);
        }
    }
    part_1();
    println!("part 2. {:?}", sum)
}

fn get_duplicates(row: &str) -> usize {
    println!("processing row: {:?}", &row);
    let (p1, p2) = row.split_at(row.len() / 2);
    return compare_groups(p1, p2);
}

fn compare_groups_part2(p1: &str, p2: &str, p3: &str) -> usize {
    println!("Comparing: {}\n,{}\n,{}\n", p1, p2 ,p3);
    let mut sum = 0;
    for char in p1.chars().collect::<HashSet<_>>() {
        if p2.contains(char) && p3.contains(char) {
            println!("Found duplicate: {:?} ", &char);
            sum += points(&char)
        }
    }
    return sum;
}

fn compare_groups(p1: &str, p2: &str) -> usize {
    let mut sum = 0;
    for char in p1.chars().collect::<HashSet<_>>() {
        if p2.contains(char) {
            println!("Found duplicate: {:?} ", &char);
            sum += points(&char)
        }
    }
    return sum;
}

fn points(char: &char) -> usize {
    let points = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    assert_eq!(points.iter().position(|x| x == &'A').unwrap() + 1, 27);
    return points.iter().position(|x| x == char).unwrap() + 1;
}