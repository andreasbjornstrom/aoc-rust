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
    let p1 = &Hands::to_type(&round[0]);
    let p2 = match target {
        "X" => { p1.get_loosing_hand() } //loose
        "Y" => { Hands::to_type(&round[0]) }// draw
        "Z" => { p1.get_winning_hand() }
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
    return count_score(&Hands::to_type(&round[0]), &Hands::to_type(&round[1]));
}


fn count_score(p1: &Hands, p2: &Hands) -> usize {
    if p1 == p2 {
        return p1.hand_value() + 3;
    }
    let score = if &p1.get_winning_hand() == p2 { 6 } else { 0 };
    return score + p2.hand_value();
}