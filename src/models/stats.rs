use serde::Serialize;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_today: i64,
    pub first_ts: i64,
    pub last_ts: i64,
    pub top_keys: Vec<(String, i64)>,
}

#[derive(Serialize)]
pub struct KeyCount {
    pub key_name: String,
    pub count: i64,
}
