pub mod post {
  use serde::{Deserialize, Serialize};
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: u32,
    pub date: String,
    pub tags: Vec<String>,
  }
}
