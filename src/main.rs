use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

#[post("/buffer")]
async fn buffer(req_body: String) -> impl Responder {
    let mut req_json: Value = serde_json::from_str(&req_body).unwrap();
    let resp: &mut serde_json::Map<String, Value> = req_json.as_object_mut().unwrap();
    resp.insert("message".to_string(), "Buffer request received!".into());
    HttpResponse::Ok().body(serde_json::to_string(resp).unwrap())
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
