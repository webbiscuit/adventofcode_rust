use std::io::{self, prelude::*};

#[derive(PartialEq, Clone, Copy, Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn what_is_beaten_by(hand_shape: HandShape) -> HandShape {
        match hand_shape {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
        }
    }

    fn what_beats(hand_shape: HandShape) -> HandShape {
        match hand_shape {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        }
    }

    fn beats_other(&self, other: HandShape) -> bool {
        *self == HandShape::what_beats(other)
    }
}

#[derive(Debug)]
struct Round {
    player1: HandShape,
    player2: HandShape,
}

impl Round {
    fn hand_score(hand_shape: HandShape) -> u32 {
        match hand_shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn player_2_score(&self) -> u32 {
        let mut score = Round::hand_score(self.player2);

        if self.player1 == self.player2 {
            score += 3;
        } else if self.player2.beats_other(self.player1) {
            score += 6;
        }

        score
    }
}

fn abc_to_hand_shape(abc: &str) -> HandShape {
    match abc {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!("Invalid hand shape"),
    }
}

fn xyz_to_hand_shape(xyz: &str) -> HandShape {
    match xyz {
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scissors,
        _ => panic!("Invalid hand shape"),
    }
}

fn xyz_to_calculated_hand_shape(xyz: &str, other_hand: HandShape) -> HandShape {
    match xyz {
        "X" => HandShape::what_is_beaten_by(other_hand),
        "Y" => other_hand,
        "Z" => HandShape::what_beats(other_hand),
        _ => panic!("Invalid hand shape"),
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines(); //.map(|l| l.unwrap()).collect();

    let mut rounds = Vec::new();
    let mut rounds_strat2 = Vec::new();

    for line in lines.map(|l| l.unwrap()) {
        let (hand1_code, hand2_code) = line.split_once(' ').unwrap();
        let hand1 = abc_to_hand_shape(hand1_code);
        let hand2 = xyz_to_hand_shape(hand2_code);

        let round = Round {
            player1: hand1,
            player2: hand2,
        };
        rounds.push(round);

        let hand2_strat2 = xyz_to_calculated_hand_shape(hand2_code, hand1);
        rounds_strat2.push(Round {
            player1: hand1,
            player2: hand2_strat2,
        });
    }

    let score = rounds.iter().map(Round::player_2_score).sum::<u32>();
    let score2 = rounds_strat2.iter().map(Round::player_2_score).sum::<u32>();

    // println!("{:?}", rounds_strat2);

    println!("You scored {} points.", score);
    println!("With secret strategy, you scored {} points.", score2);
}
