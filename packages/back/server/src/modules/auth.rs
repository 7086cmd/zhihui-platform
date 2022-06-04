pub mod auth {
  use actix_web::{post, Responder};
  #[post("/class/auth")]
  pub async fn auth_class() -> impl Responder {
    use backend_utils::utils::auth::class_account;
    class_account::create(class_account::ClassAccount {
      gradeid: 1,
      classid: 1,
      password: "123456".to_string(),
    })
    .unwrap()
  }
}
