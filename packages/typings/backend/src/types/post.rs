pub mod post {
  pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: u32,
    pub date: String,
    pub tags: Vec<String>,
  }
}
