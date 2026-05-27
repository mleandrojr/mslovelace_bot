#[derive(Default, serde::Serialize)]
pub struct BanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Default, serde::Serialize)]
pub struct RestrictParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
