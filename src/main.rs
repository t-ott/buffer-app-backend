use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/buffer")]
async fn buffer(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Buffer request received:\n{}", req_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(buffer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}