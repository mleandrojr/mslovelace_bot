use reqwest::{Client, Error};

#[derive(Default, Debug, serde::Deserialize)]
pub struct CasApiResponse {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub description: Option<String>,
}

pub struct CombotApi {
    client: Client
}

impl CombotApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn check(&self, user_id: i64) -> Result<CasApiResponse, Error> {
        let url = format!("{}?user_id={}", self.url("/check"), user_id);
        self.client.get(&url).send().await?.json().await
    }

    fn url(&self, method: &str) -> String {
        format!("https://api.cas.chat/{}", method)
    }
}
