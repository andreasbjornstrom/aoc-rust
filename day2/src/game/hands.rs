#[derive(PartialEq)]
#[derive(Debug)]
pub(crate) enum Hands {
    ROCKS,
    PAPER,
    SCISSORS,
}

pub(crate) fn hand_value(hand: &Hands) -> usize {
    return match hand {
        Hands::ROCKS => 1,
        Hands::PAPER => 2,
        Hands::SCISSORS => 3
    };
}

pub(crate) fn get_winning_hand(hand: &Hands) -> Hands {
    match hand {
        Hands::ROCKS => Hands::PAPER,
        Hands::PAPER => Hands::SCISSORS,
        Hands::SCISSORS => Hands::ROCKS
    }
}

pub(crate) fn get_loosing_hand(hand: &Hands) -> Hands {
    match hand {
        Hands::ROCKS => Hands::SCISSORS,
        Hands::PAPER => Hands::ROCKS,
        Hands::SCISSORS => Hands::PAPER
    }
}

pub(crate) fn to_type(char: &str) -> Hands {
    match char {
        "A" => Hands::ROCKS,
        "X" => Hands::ROCKS,
        "B" => Hands::PAPER,
        "Y" => Hands::PAPER,
        "C" => Hands::SCISSORS,
        "Z" => Hands::SCISSORS,
        _ => panic!("Unhandled char {:?}", char)
    }
}