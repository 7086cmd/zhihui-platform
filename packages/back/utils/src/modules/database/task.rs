pub mod task {
  use serde::{Deserialize, Serialize};
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Status {
    pub user: u32,
    pub status: String, // "Complete" | "Incomplete" | "In Progress" | "Pending" | "Cancelled"
  }
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Task {
    pub id: String,
    pub content: String,
    pub date: String,
    pub duration: f32,
    pub status: Vec<Status>,
  }
}
