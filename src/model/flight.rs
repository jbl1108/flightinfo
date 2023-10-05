use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Flight {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub airline: String,
    pub code : String,
}