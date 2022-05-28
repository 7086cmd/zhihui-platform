extern crate mongodb;

use mongodb::{options::ClientOptions, Client};

pub fn tryit() {
  let mut client_options =
    ClientOptions::parse("mongodb://localhost:27017").expect("Failed to parse URI");
  client_options.app_name = Some("actix-web-mongodb-example");

  let client = Client::with_options(client_options).expect("Failed to initialize client");

  for db_name in client
    .list_database_names(None)
    .await
    .expect("Failed to list databases")
  {
    println!("{}", db_name);
  }
}

fn main() {
  tryit();
}
