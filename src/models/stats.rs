use serde::Serialize;

#[derive(Serialize)]
pub struct DashboardStats {
    pub total: i64,
}
