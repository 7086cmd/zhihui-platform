pub mod member {
  pub fn check_password(number: u32, password: String) -> bool {
    use super::super::database::member::Member;
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .expect("Failed to initialize standalone client.");
    let collection = client.database("magnifique").collection::<Member>("member");
    let result = collection.find_one(Some(doc! { "number": number }), None);
    use base64::encode;
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
    use mongodb::bson::doc;
    let based_str: String = encode(password.as_bytes());
    let mut hash_str: Sha512 = Sha512::new();
    // hash_str.input_str(based_str);
    hash_str.input(based_str.as_bytes());
    let password_result = hash_str.result_str();
    match result {
      Ok(result) => {
        let member = result.unwrap();
        let member_password = member.password.clone();
        if member_password == password_result {
          return true;
        }
      }
      Err(_) => {}
    }
    false
  }
}
