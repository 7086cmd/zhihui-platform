pub mod deduction {
  use serde::{Deserialize, Serialize};
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Deduction {
    pub person: u32,
    pub deduction: f32,
    pub reason: String,
    pub date: String,
    pub place: String,
    pub deductor: u32,
    pub description: String,
    pub id: String,
  }
}
