use std::cmp::Ordering;
use std::io;

struct ScoreState {
    serving_team: TeamFlag,
    serving_rank: ServingRank,
    score_red: u8,
    score_blue: u8,
    grid_red: (Player, Player),
    grid_blue: (Player, Player),
}

enum TeamFlag {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone)]

enum Player {
    A,
    B,
}

enum ServingRank {
    First,
    Second,
}

fn flip_grid(grid: (Player, Player)) -> (Player, Player) {
    (grid.1, grid.0)
}

fn point(state: &mut ScoreState) {
    match state.serving_team {
        TeamFlag::Red => {
            state.score_red += 1;
            state.grid_red = flip_grid(state.grid_red);
        }
        TeamFlag::Blue => {
            state.score_blue += 1;
            state.grid_blue = flip_grid(state.grid_blue);
        }
    }
    println!("Point.")
}

fn break_point(state: &mut ScoreState) {
    match state.serving_rank {
        ServingRank::First => {
            state.serving_rank = ServingRank::Second;
            println!("[Block Point]: 2nd Serve");
        }
        ServingRank::Second => {
            state.serving_rank = ServingRank::First;
            match state.serving_team {
                TeamFlag::Red => {
                    state.serving_team = TeamFlag::Blue;
                }
                TeamFlag::Blue => {
                    state.serving_team = TeamFlag::Red;
                }
            }
            println!("[Break Point]: Side Out");
        }
    }
}

fn main() {
    let mut state = ScoreState {
        serving_team: TeamFlag::Red,
        serving_rank: ServingRank::Second,
        score_red: 0,
        score_blue: 0,
        grid_blue: (Player::B, Player::A),
        grid_red: (Player::B, Player::A),
    };

    println!("Game Start!");
    println!("Score: 0, 0, 2");
    println!("       Blu{:?}", state.grid_blue);
    println!("       Red{:?} <- Serving Side", state.grid_red);

    loop {
        let mut input = String::new();
        println!("");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u8 = input.trim().parse().expect("Input must be a number");

        let input: TeamFlag = match input.cmp(&0) {
            Ordering::Less | Ordering::Equal => TeamFlag::Blue,
            Ordering::Greater => TeamFlag::Red,
        };

        match state.serving_team {
            TeamFlag::Blue => match input {
                TeamFlag::Blue => point(&mut state),
                TeamFlag::Red => break_point(&mut state),
            },
            TeamFlag::Red => match input {
                TeamFlag::Red => point(&mut state),
                TeamFlag::Blue => break_point(&mut state),
            },
        };

        let f_score = match state.serving_team {
            TeamFlag::Red => &state.score_red,
            TeamFlag::Blue => &state.score_blue,
        };

        let s_score = match state.serving_team {
            TeamFlag::Red => &state.score_blue,
            TeamFlag::Blue => &state.score_red,
        };

        let r_score = match state.serving_rank {
            ServingRank::First => 1,
            ServingRank::Second => 2,
        };

        println!("Score: {}, {}, {}", f_score, s_score, r_score);
        match state.serving_team {
            TeamFlag::Red => {
                println!("       Blu{:?}", state.grid_blue);
                println!("       Red{:?}", state.grid_red);
            }
            TeamFlag::Blue => {
                println!("       Red{:?}", state.grid_red);
                println!("       Blu{:?}", state.grid_blue);
            }
        }
    }
}
