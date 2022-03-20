use crate::options::Difficulty;

pub struct Score {
    pub name: String,
    pub level: usize,
    pub difficulty: Difficulty,
    pub time: f32,
}

pub fn get_scores(level: usize, difficulty: Difficulty) -> Vec<Score> {
    //TODO: fetch from backend
    vec![
        Score {
            name: "luiz".to_string(),
            level,
            difficulty,
            time: 6.0,
        },
        Score {
            name: "jorge".to_string(),
            level,
            difficulty,
            time: 4.2,
        },
    ]
}