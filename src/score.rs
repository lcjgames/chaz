use bevy::tasks::AsyncComputeTaskPool;

use crate::options::Difficulty;

#[derive(Clone, Debug, Deserialize)]
pub struct Score {
    pub id: i32,
    pub high_score: i32,
    pub username: String,
    pub difficulty: Difficulty,
    pub level: String,
}

static mut RESULT: Vec<Score> = Vec::new(); // TODO: I know, I'm ashamed of this too

pub fn get_scores(level: String, difficulty: Difficulty, task_pool: &AsyncComputeTaskPool) -> Vec<&Score> {
    unsafe { //TODO: hacky hack
        task_pool.spawn(async move {
            let fetch = fetch().await;
            RESULT = match fetch {
                Ok(v) => v,
                Err(e) => {
                    crate::log::console_log!("Fetch error: {:?}", e);
                    Vec::new()
                },
            }
        }).detach();
        RESULT.iter().filter(
            |score| score.level == level && score.difficulty == difficulty
        ).collect()
    }
}

pub async fn fetch() -> Result<Vec<Score>, reqwest::Error> {
    use reqwest::Client;

    let res = Client::new()
        .get("http://127.0.0.1:8001/highscores/list")
        .header("Accept", "application/vnd.github.v3+json")
        .header("Access-Control-Allow-Origin", "Any")
        .send()
        .await?;

    Ok(res.json().await?)
}
