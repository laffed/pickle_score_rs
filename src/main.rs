use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let mut score = Score::new();

    loop {
        let mut input = String::new();
        score.announce_score();

        println!("input point winner:");
        stdin().read_line(&mut input).expect("Failed to read input");
        let point_winner = is_valid_input(&input);

        match point_winner {
            Some(pw) => {
                score.calc_point(pw);
            }
            None => {
                println!("Invalid input. Please try again.");
                continue;
            }
        }

        match score.is_game_over() {
            Some(winner) => {
                println!("Game over! Winner is: {:?}", winner);
                break;
            }
            None => continue,
        }
    }
}

const SCORE_SOFT_CAP: u32 = 11;
const DUECE_WIN_DIFF: u32 = 2;

const VALID_SERV_INPUT: [&str; 3] = ["0", "serv", "serving"];
const VALID_REC_INPUT: [&str; 3] = ["1", "rec", "receiving"];

fn is_valid_input(input: &str) -> Option<RelativeSide> {
    let input = input.trim();
    for s in VALID_SERV_INPUT.iter() {
        if input == *s {
            return Some(RelativeSide::Serving);
        }
    }
    for r in VALID_REC_INPUT.iter() {
        if input == *r {
            return Some(RelativeSide::Receiving);
        }
    }

    None
}

struct Score {
    serving: Server,
    red: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Server {
    Red1,
    Red2,
    Blue1,
    Blue2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Team {
    Red,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RelativeSide {
    Serving,
    Receiving,
}

impl Score {
    pub fn new() -> Self {
        Score {
            serving: Server::Red2,
            red: 0,
            blue: 0,
        }
    }

    pub fn announce_score(&self) {
        match self.serving {
            Server::Red1 => println!("Score: Red {}, {}, {}", self.red, self.blue, 1),
            Server::Red2 => println!("Score: Red {}, {}, {}", self.red, self.blue, 2),
            Server::Blue1 => println!("Score: Blue {}, {}, {}", self.blue, self.red, 1),
            Server::Blue2 => println!("Score: Blue {}, {}, {}", self.blue, self.red, 2),
        };
    }

    fn calc_next_server(&self, point_winner: RelativeSide) -> Server {
        match &self.serving {
            Server::Red1 => match point_winner {
                RelativeSide::Serving => Server::Red1,
                RelativeSide::Receiving => Server::Red2,
            },
            Server::Red2 => match point_winner {
                RelativeSide::Serving => Server::Red2,
                RelativeSide::Receiving => Server::Blue1,
            },
            Server::Blue1 => match point_winner {
                RelativeSide::Serving => Server::Blue1,
                RelativeSide::Receiving => Server::Blue2,
            },
            Server::Blue2 => match point_winner {
                RelativeSide::Serving => Server::Blue2,
                RelativeSide::Receiving => Server::Red1,
            },
        }
    }

    pub fn is_game_over(&self) -> Option<Team> {
        if self.red < SCORE_SOFT_CAP && self.blue < SCORE_SOFT_CAP {
            return None;
        };

        let leading_team = match self.red.cmp(&self.blue) {
            Ordering::Greater => Team::Red,
            Ordering::Less => Team::Blue,
            Ordering::Equal => return None,
        };

        match self.red.abs_diff(self.blue) >= DUECE_WIN_DIFF {
            false => None,
            true => Some(leading_team),
        }
    }

    pub fn calc_point(&mut self, point_winner: RelativeSide) {
        let increment: u32 = if point_winner == RelativeSide::Serving {
            1
        } else {
            0
        };

        match self.serving {
            Server::Red1 | Server::Red2 => self.red += increment,
            Server::Blue1 | Server::Blue2 => self.blue += increment,
        };

        self.serving = self.calc_next_server(point_winner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_input() {
        assert_eq!(is_valid_input("0"), Some(RelativeSide::Serving));
        assert_eq!(is_valid_input("serv"), Some(RelativeSide::Serving));
        assert_eq!(is_valid_input("serving"), Some(RelativeSide::Serving));
        assert_eq!(is_valid_input("1"), Some(RelativeSide::Receiving));
        assert_eq!(is_valid_input("rec"), Some(RelativeSide::Receiving));
        assert_eq!(is_valid_input("receiving"), Some(RelativeSide::Receiving));
        assert_eq!(is_valid_input("invalid"), None);
    }

    #[test]
    fn test_calc_next_server() {
        let score = Score {
            serving: Server::Red1,
            red: 0,
            blue: 0,
        };
        assert_eq!(score.calc_next_server(RelativeSide::Serving), Server::Red1);
        assert_eq!(
            score.calc_next_server(RelativeSide::Receiving),
            Server::Red2
        );

        let score = Score {
            serving: Server::Red2,
            red: 0,
            blue: 0,
        };
        assert_eq!(score.calc_next_server(RelativeSide::Serving), Server::Red2);
        assert_eq!(
            score.calc_next_server(RelativeSide::Receiving),
            Server::Blue1
        );

        let score = Score {
            serving: Server::Blue1,
            red: 0,
            blue: 0,
        };
        assert_eq!(score.calc_next_server(RelativeSide::Serving), Server::Blue1);
        assert_eq!(
            score.calc_next_server(RelativeSide::Receiving),
            Server::Blue2
        );

        let score = Score {
            serving: Server::Blue2,
            red: 0,
            blue: 0,
        };
        assert_eq!(score.calc_next_server(RelativeSide::Serving), Server::Blue2);
        assert_eq!(
            score.calc_next_server(RelativeSide::Receiving),
            Server::Red1
        );
    }

    #[test]
    fn is_game_over_true() {
        let score = Score {
            serving: Server::Red1,
            red: 11,
            blue: 9,
        };
        assert_eq!(score.is_game_over(), Some(Team::Red));
        let score = Score {
            serving: Server::Blue1,
            red: 9,
            blue: 11,
        };
        assert_eq!(score.is_game_over(), Some(Team::Blue));
        let score = Score {
            serving: Server::Red2,
            red: 14,
            blue: 12,
        };
        assert_eq!(score.is_game_over(), Some(Team::Red));
        let score = Score {
            serving: Server::Blue2,
            red: 20,
            blue: 22,
        };
        assert_eq!(score.is_game_over(), Some(Team::Blue));
    }

    #[test]
    fn is_game_over_false() {
        let score = Score {
            serving: Server::Red1,
            red: 10,
            blue: 9,
        };
        assert_eq!(score.is_game_over(), None);
        let score = Score {
            serving: Server::Blue1,
            red: 9,
            blue: 10,
        };
        assert_eq!(score.is_game_over(), None);
        let score = Score {
            serving: Server::Red2,
            red: 13,
            blue: 12,
        };
        assert_eq!(score.is_game_over(), None);
        let score = Score {
            serving: Server::Blue2,
            red: 21,
            blue: 22,
        };
        assert_eq!(score.is_game_over(), None);
    }
}
