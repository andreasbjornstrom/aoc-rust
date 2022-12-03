mod game;

extern crate core;

use std::io::empty;
use game::hands::Hands;
use crate::game::hands;

fn main() {
    let data = "A Y
B X
C Z".to_string();
    let data = aoc::get_aoc_file(2);
    let result: usize = data.split("\n")
        .map(|t| process_turn_second_part(t.split(" ").collect())).sum();
    println!("part 1: {:?}", part_1());
    println!("part 2: {:?}", result)
}

fn process_turn_second_part(round: Vec<&str>) -> usize {
    if round.len() < 2 {
        return 0;
    }
    let target = round[1];
    let p1 = &hands::to_type(&round[0]);
    let p2 = match target {
        "X" => { hands::get_loosing_hand(p1) } //loose
        "Y" => { hands::to_type(&round[0]) }// draw
        "Z" => { hands::get_winning_hand(p1) }
        _ => panic!("unknown")
    };
    let score = count_score(p1, &p2);
    println!("p1: {:?}, p2 {:?}, with score: {:?}", p1, &p2, score);
    return score;
}


fn part_1() -> usize {
    let data = "A Y
B X
C Z".to_string();
    let data = aoc::get_aoc_file(2);
    let result: usize = data.split("\n")
        .map(|t| process_turn(t.split(" ").collect())).sum();
    return result;
}


fn process_turn(round: Vec<&str>) -> usize {
    if round.len() < 2 {
        return 0;
    }
    dbg!(&round);
    return count_score(&hands::to_type(&round[0]), &hands::to_type(&round[1]));
}


fn count_score(p1: &Hands, p2: &Hands) -> usize {
    if p1 == p2 {
        return hands::hand_value(&p1) + 3;
    }
    let score = if &hands::get_winning_hand(p1) == p2 { 6 } else { 0 };
    return score + hands::hand_value(p2);
}