pub mod deduction {
  pub fn delete(id: String) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client
      .database("magnifique")
      .collection::<types::typical::deduction::Deduction>("deduction");
    let result = collection.delete_one(
      mongodb::bson::doc! {
        "id": id
      },
      None,
    );
    match result {
      Ok(result) => {
        if result.deleted_count == 1 {
          Ok("Deduction deleted".to_string())
        } else {
          Err("Failed to delete deduction".to_string())
        }
      }
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn create(deduction: types::typical::deduction::Deduction) -> Result<String, String> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())?;
    let collection = client.database("magnifique").collection("deduction");
    let result = collection.insert_one(
      mongodb::bson::doc! {
          "person": &deduction.person,
          "deduction": &deduction.deduction,
          "date": &deduction.date,
          "id": &deduction.id,
          "description": &deduction.description,
          "reason": &deduction.reason,
          "place": &deduction.place,
          "deductor": &deduction.deductor,
      },
      None,
    );
    match result {
      Ok(result) => Ok(result.inserted_id.to_string()),
      Err(error) => Err(error.to_string()),
    }
  }
  pub fn list() -> Option<Vec<types::typical::deduction::Deduction>> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
      .map_err(|error| error.to_string())
      .ok()?;
    let collection = client
      .database("magnifique")
      .collection::<types::typical::deduction::Deduction>("deduction");
    let mut result: Vec<types::typical::deduction::Deduction> = Vec::new();
    for item in collection.find(None, None).ok()? {
      result.push(item.unwrap());
    }
    Some(result)
  }
}
