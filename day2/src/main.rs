use std::vec::Vec;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    fn check_win(&self, rhs: &RockPaperScissors) -> bool {
        self.loses_against().eq(rhs)
    }

    fn wins_against(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        }
    }

    fn loses_against(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper
        }
    }
}

impl From<RockPaperScissors> for i32 {
    fn from(rps: RockPaperScissors) -> i32 {
        match rps {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3
        }
    }
} 

#[derive(Debug, PartialEq, Eq)]
struct RockPaperScissorsError;

impl FromStr for RockPaperScissors {
    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'A' | 'X' => Ok(RockPaperScissors::Rock),
            'B' | 'Y' => Ok(RockPaperScissors::Paper),
            'C' | 'Z' => Ok(RockPaperScissors::Scissors),
            _ => Err(RockPaperScissorsError)
        }
    }
}

#[derive(Clone, Copy)]
enum RoundShould {
    Win,
    Draw,
    Lose
}

struct Round {
    lhs: RockPaperScissors,
    rhs: RockPaperScissors,
    should: RoundShould,
}

impl Round {
    fn result(&self) -> bool {
        self.rhs.check_win(&self.lhs)
    }

    fn points(&self) -> i32 {
        let rhs: i32 = self.rhs.into();
        if self.result() {
            6 + rhs
        } else if self.lhs == self.rhs {
            3 + rhs
        } else {
            rhs
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RoundErr;

impl FromStr for Round {
    type Err = RoundErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lhs = RockPaperScissors::from_str(&s[0..1]);
        let rhs = RockPaperScissors::from_str(&s[2..3]);

        let should = match &s[2..3] {
            "X" => Ok(RoundShould::Lose),
            "Y" => Ok(RoundShould::Draw),
            "Z" => Ok(RoundShould::Win),
            _ => Err(RoundErr)
        };

        if lhs.is_err() || rhs.is_err() || should.is_err() {
            Err(RoundErr)
        } else {
            Ok(Round { lhs: lhs.unwrap(), rhs: rhs.unwrap(), should: should.unwrap() })
        }
    }

}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let rounds: Vec<Round> = input.split('\n')
                                  .filter(|x| x.len() != 0)
                                  .map(|x| Round::from_str(x).unwrap())
                                  .collect();

    let total_score: i32 = rounds.iter()
                                 .map(|x| x.points()) 
                                 .sum();

    println!("total score (part 1): {:?}", total_score);

    let total_score_2: i32 = rounds.iter()
                                    .map(|x| Round { 
                                        lhs: x.lhs, 
                                        rhs: match x.should {
                                            RoundShould::Draw => x.lhs,
                                            RoundShould::Lose => x.lhs.loses_against(),
                                            RoundShould::Win => x.lhs.wins_against()
                                        },
                                        should: x.should 
                                    }.points())
                                    .sum();

    println!("total score (part 2): {:?}", total_score_2);
}
