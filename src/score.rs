use wasm_bindgen_futures::spawn_local;

use crate::options::Difficulty;

pub struct Score {
    pub name: String,
    pub level: usize,
    pub difficulty: Difficulty,
    pub time: f32,
}

pub async fn fetch() -> Result<String, reqwest::Error> {
    let res = reqwest::Client::new()
        .get("http://127.0.0.1:8001/highscores/list")
        .header("Accept", "application/vnd.github.v3+json")
        .header("Access-Control-Allow-Origin", "Any")
        .send()
        .await?;

    Ok(res.text().await?)
}

pub fn get_scores(level: usize, difficulty: Difficulty) -> Vec<Score> {
    spawn_local(async {
        let scores = fetch().await;
        crate::log::console_log!("{:?}", scores);
    });
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