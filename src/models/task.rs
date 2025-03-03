use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  id: uuid::Uuid,
  name: String,
  status: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl Task {
  fn new(name: String) -> Self {
    Task {
      id: Uuid::new_v4(),
      name,
      status: "pending".to_string(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }

  fn update_task() {}
}
