pub mod member {
  use types::typical::member::Member;
  pub fn read_single_member_with_number(number: u32) -> Option<Member> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
    for i in collection.find(Nond, None).ok()? {
      if i.as_ref().unwrap().number == number {
        return Some(i.as_ref().unwrap().clone());
      }
    }
  }
  pub fn delete(number: u32) -> Result<std::string::String, mongodb::error::Error> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
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
  pub fn create(member: Member) -> Result<std::string::String, std::error::Error> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
    let result = collection.insert_one(member, None);
    match result {
      Ok(result) => {
        if result.inserted_count == 1 {
          Ok("Member created".to_string())
        } else {
          Err("Failed to create member".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
    // Ok(())
  }
}
