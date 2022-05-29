pub mod task {
  pub struct Status {
    pub user: u32,
    pub status: String, // "Complete" | "Incomplete" | "In Progress" | "Pending" | "Cancelled"
  }
  pub struct Task {
    pub id: String,
    pub content: String,
    pub date: String,
    pub duration: f32,
    pub status: Vec<Status>,
  }
}
