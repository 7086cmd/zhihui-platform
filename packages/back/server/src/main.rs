use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/api/v1")]
async fn v1() -> impl Responder {
  HttpResponse::Ok().body("v1")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index).service(v1))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
