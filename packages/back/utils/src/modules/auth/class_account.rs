pub mod class_account {
  use serde::{Deserialize, Serialize};
  #[derive(Serialize, Deserialize, Debug)]
  pub struct ClassAccount {
    pub gradeid: u32,
    pub classid: u32,
    pub password: String,
  }

  pub fn create(classaccount: ClassAccount) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<ClassAccount>("classaccount");
    use mongodb::bson::doc;
    let one = collection.find_one(
      Some(doc! {
        "gradeid": &classaccount.gradeid,
        "classid": &classaccount.classid,
      }),
      None,
    );
    match one {
      Ok(_data) => {
        return Err("Class account already exists".to_string());
      }
      _ => {}
    }
    let result = collection.insert_one(classaccount, None);
    match result {
      Ok(result) => Ok(result.inserted_id.to_string()),
      Err(error) => Err(error.to_string()),
    }
  }

  pub fn check_password(gradeid: u32, classid: u32, password: String) -> bool {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .expect("Failed to initialize standalone client.");
    let collection = client
      .database("magnifique")
      .collection::<ClassAccount>("classaccount");
    use mongodb::bson::doc;
    let result = collection.find_one(
      Some(doc! {
        "gradeid": gradeid,
        "classid": classid
      }),
      None,
    );
    use base64::encode;
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
    let based_str: String = encode(password.as_bytes());
    let mut hash_str: Sha512 = Sha512::new();
    // hash_str.input_str(based_str);
    hash_str.input(based_str.as_bytes());
    let password_result = hash_str.result_str();
    match result {
      Ok(result) => {
        let classaccount = result.unwrap();
        let classaccount_password = classaccount.password.clone();
        if classaccount_password == password_result {
          return true;
        }
      }
      Err(_) => {}
    }
    false
  }
  pub fn edit_password(gradeid: u32, classid: u32, password: String) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<ClassAccount>("classaccount");
    use base64::encode;
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
    use mongodb::bson::doc;
    let based_str: String = encode(password.as_bytes());
    let mut hash_str: Sha512 = Sha512::new();
    // hash_str.input_str(based_str);
    hash_str.input(based_str.as_bytes());
    let password_result = hash_str.result_str();
    let result = collection.update_one(
      doc! {
        "gradeid": gradeid,
        "classid": classid
      },
      doc! {
        "$set": {
          "password": password_result
        }
      },
      None,
    );
    match result {
      Ok(result) => Ok(result.modified_count.to_string()),
      Err(error) => Err(error.to_string()),
    }
  }
}
