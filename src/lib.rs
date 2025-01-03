use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
// async fn greet(req: HttpRequest) -> impl Responder {
// let name = req.match_info().get("name").unwrap_or("World");
// format!("Hello {}!", &name)
// }
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
// no await here
    Ok(server)
}