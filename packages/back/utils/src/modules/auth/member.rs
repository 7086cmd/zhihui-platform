pub mod member {
  pub fn check_password(
    number: u32,
    password: String,
  ) -> Result<bool, std::string::String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/").unwrap();
    let collection = client.database("magnifique").collection::<Member>("member");
    use base64::encode;
    use crypto::sha2::Sha512;
    const based_str = encode(password);
    const hash_str = Sha512::new();
    hash_str.input_str(based_str);
    const password_result = hash_str.result_str();
    let result = collection.find_one(
      doc! {"number": number},
      None,
    );
    match result {
      Ok(result) => {
        if result.is_some() {
          let member = result.unwrap();
          if member.as_ref().unwrap().password == password_result {
            Ok(true)
          } else {
            Ok(false)
          }
        } else {
          Ok(false)
        }
      }
      Err(error) => Err(error.to_string()),
    }
  }
}
