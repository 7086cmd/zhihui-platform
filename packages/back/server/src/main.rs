#[macro_use]
extern crate rocket;

#[get("/")]
fn init() -> String {
  format!(
    "{{
    \"a\": \"b\"
  }}"
  )
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![init])
}
