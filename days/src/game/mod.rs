#[derive(PartialEq, Debug)]
pub enum Hands {
    ROCKS,
    PAPER,
    SCISSORS,
}

impl Hands {
    pub fn hand_value(&self) -> usize {
        match &self {
            Hands::ROCKS => 1,
            Hands::PAPER => 2,
            Hands::SCISSORS => 3,
        }
    }

    pub fn get_winning_hand(&self) -> Hands {
        match &self {
            Hands::ROCKS => Hands::PAPER,
            Hands::PAPER => Hands::SCISSORS,
            Hands::SCISSORS => Hands::ROCKS,
        }
    }

    pub fn get_loosing_hand(&self) -> Hands {
        match &self {
            Hands::ROCKS => Hands::SCISSORS,
            Hands::PAPER => Hands::ROCKS,
            Hands::SCISSORS => Hands::PAPER,
        }
    }

    pub fn to_type(char: &str) -> Hands {
        match char {
            "A" => Hands::ROCKS,
            "X" => Hands::ROCKS,
            "B" => Hands::PAPER,
            "Y" => Hands::PAPER,
            "C" => Hands::SCISSORS,
            "Z" => Hands::SCISSORS,
            _ => panic!("Unhandled char {:?}", char),
        }
    }
}
