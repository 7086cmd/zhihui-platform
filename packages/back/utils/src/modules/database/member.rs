extern crate crypto;

pub mod member {
  use super::deduction::Deduction;
  use super::post::Post;
  use super::task::Task;
  use serde::{Deserialize, Serialize};
  #[derive(Serialize, Deserialize, Debug)]
  struct Register {
    pub prize: String,
    pub plan: String,
    pub position: String,
    pub introduce: String,
  }
  #[derive(Serialize, Deserialize, Debug)]
  struct Violation {
    pub reason: String,
    pub date: String,
    pub actioner: String,
    pub description: String,
  }
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Member {
    pub name: String,
    pub number: u32,
    pub deduction: Vec<Deduction>,
    pub posts: Vec<Post>,
    pub task: Vec<Task>,
    pub password: String, // base64 + sha512
  }

  pub fn read_single_member_with_number(number: u32) -> Option<Member> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").ok()?;
    let collection = client.database("magnifique").collection::<Member>("member");
    for i in collection.find(None, None).ok()? {
      if i.as_ref().unwrap().number == number {
        return Some(i.unwrap());
      }
    }
    None
  }
  pub fn delete(number: u32) -> Result<std::string::String, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
    use mongodb::bson::doc;
    let result = collection.delete_one(doc! {"number": number}, None);
    match result {
      Ok(result) => {
        if result.deleted_count == 1 {
          Ok("Member deleted".to_string())
        } else {
          Err("Failed to delete member".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
    // Ok(())
  }
  pub fn create(member: Member) -> Result<std::string::String, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
    let number = member.number;
    let result = collection.insert_one(member, None);
    edit_password(number, number.to_string()).unwrap();
    match result {
      Ok(_result) => Ok("Member created".to_string()),
      Err(error) => Err(error.to_string()),
    }
    // Ok(())
  }
  pub fn edit_password(
    number: u32,
    password: String,
  ) -> Result<std::string::String, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
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
      doc! {"number": number},
      doc! {"$set": {"password": password_result}},
      None,
    );
    match result {
      Ok(result) => {
        if result.modified_count == 1 {
          Ok("Password changed".to_string())
        } else {
          Err("Failed to change password".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
    // Ok(())
  }
}
