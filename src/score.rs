use bevy::tasks::AsyncComputeTaskPool;

use crate::options::Difficulty;

pub struct Score {
    pub name: String,
    pub level: usize,
    pub difficulty: Difficulty,
    pub time: f32,
}

static mut RESULT: String = String::new(); // TODO: I know, I'm ashamed of this too

pub fn get_scores(level: usize, difficulty: Difficulty, task_pool: &AsyncComputeTaskPool) -> Vec<Score> {
    unsafe { //TODO: hacky hack
        task_pool.spawn(async move {
            let fetch = fetch().await;
            RESULT = match fetch {
                Ok(s) => s,
                Err(e) => e.to_string(),
            }
        }).detach();
        crate::log::console_log!("scores: {:?}", RESULT);
    }
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

pub async fn fetch() -> Result<String, reqwest::Error> {
    use reqwest::Client;

    let res = Client::new()
        .get("http://127.0.0.1:8001/highscores/list")
        .header("Accept", "application/vnd.github.v3+json")
        .header("Access-Control-Allow-Origin", "Any")
        .send()
        .await?;

    Ok(res.text().await?)
}
