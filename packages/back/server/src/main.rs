include!("modules/auth.rs");

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
  HttpResponse::Ok().body(format!("Hello world!, {:?}", req))
}

#[get("/api/v1")]
async fn v1() -> impl Responder {
  HttpResponse::Ok().body("v1")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(index)
      .service(v1)
      .service(auth::auth_class)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
