pub mod token {
  use serde::{Deserialize, Serialize};
  use uuid::Uuid;
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Token {
    pub token: String,
    pub username: String,
    pub password: String,
    pub date: i64,
  }
  pub fn create(username: String, password: String) -> Result<String, String> {
    let token_content = Uuid::new_v4().to_string();
    let date = chrono::Local::now().timestamp();
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client.database("magnifique").collection::<Token>("token");
    let token = Token {
      token: token_content.clone(),
      username: username.clone(),
      password: password.clone(),
      date: date.clone(),
    };
    let result = collection.insert_one(token, None);
    match result {
      Ok(_result) => Ok(token_content),
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn delete(token: String) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client.database("magnifique").collection::<Token>("token");
    use mongodb::bson::doc;
    let result = collection.delete_one(doc! {"token": token}, None);
    match result {
      Ok(result) => {
        if result.deleted_count == 1 {
          Ok("Token deleted.".to_string())
        } else {
          Err("Failed to delete token.".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn check_usablity(token: String) -> bool {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())
      .unwrap();
    let collection = client.database("magnifique").collection::<Token>("token");
    for i in collection
      .find(None, None)
      .map_err(|error| error.to_string())
      .unwrap()
    {
      if i.as_ref().unwrap().token == token.clone() {
        let date_now = chrono::Local::now().timestamp();
        if (date_now - i.as_ref().unwrap().date) < (30 * 24 * 60 * 60) {
          return true;
        } else {
          delete(token).unwrap();
          return false;
        }
      }
    }
    false
  }
}
