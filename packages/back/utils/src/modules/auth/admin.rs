pub mod admin {
  use serde::{Deserialize, Serialize};
  #[derive(Serialize, Deserialize, Debug)]
  pub struct AdminAccount {
    pub username: String,
    pub password: String,
  }
  pub fn create(adminaccount: AdminAccount) -> Result<std::string::String, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<AdminAccount>("adminaccount");
    let user = &adminaccount.username.clone();
    let result = collection.insert_one(adminaccount, None);
    edit_password(user.to_string(), "12345678".to_string()).unwrap();
    match result {
      Ok(_result) => Ok("Admin created.".to_string()),
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn delete(username: String) -> Result<std::string::String, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<AdminAccount>("adminaccount");
    use mongodb::bson::doc;
    let result = collection.delete_one(doc! {"username": username}, None);
    match result {
      Ok(result) => {
        if result.deleted_count == 1 {
          Ok("Admin deleted.".to_string())
        } else {
          Err("Failed to delete admin.".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn check_password(username: String, password: String) -> bool {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())
      .unwrap();
    let collection = client
      .database("magnifique")
      .collection::<AdminAccount>("adminaccount");
    for i in collection
      .find(None, None)
      .map_err(|error| error.to_string())
      .unwrap()
    {
      if i.as_ref().unwrap().username == username {
        if i.as_ref().unwrap().password == password {
          return true;
        }
      }
    }
    false
  }
  pub fn edit_password(username: String, password: String) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<AdminAccount>("adminaccount");
    use base64::encode;
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
    use mongodb::bson::doc;
    let based_str: String = encode(password.as_bytes());
    let mut hash_str: Sha512 = Sha512::new();
    hash_str.input(based_str.as_bytes());
    let password_result = hash_str.result_str();
    let result = collection.find_one_and_update(
      doc! {
        "username": username
      },
      doc! {
        "$set": {
          "password": password_result
        }
      },
      None,
    );
    match result {
      Ok(_result) => Ok("Password edited.".to_string()),
      Err(error) => Err(error.to_string()),
    }
  }
}
