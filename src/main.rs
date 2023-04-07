
use actix_web::{ get, App, Responder, HttpResponse, HttpServer };

#[get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("This is home")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
      App::new()
        .service(home)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
