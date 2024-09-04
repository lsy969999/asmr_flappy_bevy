use desktop::{load, save};
use serde::{Deserialize, Serialize};

mod desktop;

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub score: u32,
}

pub fn get_score() -> Score {
    let score_str = load("score");
    let score = serde_json::from_str::<Score>(&score_str).unwrap_or(Score { score: 0 });
    score
}

pub fn set_score(score: Score) {
    let score_str = serde_json::to_string(&score).unwrap();
    save("score", &score_str);
}

#[test]
fn get_score_test() {
    let score = get_score();
    println!("score : {:?}", score);
}

#[test]
fn set_score_test() {
    let score = Score { score: 0 };
    set_score(score);
}
