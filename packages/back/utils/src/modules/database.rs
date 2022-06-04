pub mod database {
  include!("database/deduction.rs");
  include!("database/member.rs");
  include!("database/post.rs");
  include!("database/task.rs");
  pub fn init() {
    mod init {
      use serde::{Deserialize, Serialize};
      #[derive(Serialize, Deserialize, Debug)]
      struct AdminAccount {
        pub username: String,
        pub password: String,
      }
      pub fn __admin_create__() -> Result<std::string::String, std::string::String> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
          .map_err(|error| error.to_string())?;
        let collection = client
          .database("magnifique")
          .collection::<AdminAccount>("adminaccount");
        use mongodb::bson::doc;
        let result = collection.find_one(
          Some(doc! {
            "username": "admin",
          }),
          None,
        );
        match result {
          Ok(_data) => {
            return Err("Admin account already exists".to_string());
          }
          _ => {}
        }
        use base64::encode;
        use crypto::digest::Digest;
        use crypto::sha2::Sha512;
        let based_str: String = encode("123456".as_bytes());
        let mut hash_str: Sha512 = Sha512::new();
        hash_str.input(based_str.as_bytes());
        let password_result = hash_str.result_str();
        let result = collection.insert_one(
          AdminAccount {
            username: "admin".to_string(),
            password: password_result,
          },
          None,
        );
        match result {
          Ok(_result) => Ok("Admin created.".to_string()),
          Err(error) => Err(error.to_string()),
        }
      }
      pub fn __admin_edit_password__() -> Result<String, String> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
          .map_err(|error| error.to_string())?;
        let collection = client
          .database("magnifique")
          .collection::<AdminAccount>("adminaccount");
        use base64::encode;
        use crypto::digest::Digest;
        use crypto::sha2::Sha512;
        use mongodb::bson::doc;
        let based_str: String = encode("123456".as_bytes());
        let mut hash_str: Sha512 = Sha512::new();
        hash_str.input(based_str.as_bytes());
        let password_result = hash_str.result_str();
        let result = collection.find_one_and_update(
          doc! {
            "username": "admin"
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
      #[derive(Serialize, Deserialize, Debug)]
      pub struct ClassAccount {
        pub gradeid: u32,
        pub classid: u32,
        pub password: String,
      }

      pub fn __class_create__() -> Result<(), String> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
          .map_err(|error| error.to_string())?;
        let collection = client
          .database("magnifique")
          .collection::<ClassAccount>("classaccount");
        use mongodb::bson::doc;
        for i in 1..3 {
          for j in 1..15 {
            use base64::encode;
            use crypto::digest::Digest;
            use crypto::sha2::Sha512;
            use mongodb::bson::doc;
            let based_str: String = encode("123456".as_bytes());
            let mut hash_str: Sha512 = Sha512::new();
            hash_str.input(based_str.as_bytes());
            let password_result = hash_str.result_str();
            let res = collection.find_one(
              Some(doc! {
                "gradeid": i,
                "classid": j,
              }),
              None,
            );
            if let Err(_v) = res {
              let result = collection.insert_one(
                ClassAccount {
                  gradeid: i,
                  classid: j,
                  password: password_result.to_string(),
                },
                None,
              );
              match result {
                Err(error) => return Err(error.to_string()),
                _ => {}
              }
            }
          }
        }
        Ok(())
      }
    }
    init::__admin_create__().unwrap();
    init::__class_create__().unwrap();
  }
}
