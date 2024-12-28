use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod router;
mod services;
mod word_provider;
mod score;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(router::start_game)
            .service(router::guess)
            .service(router::get_users)
            .service(router::get_user_word)
            .service(router::get_user_score)
            .service(router::set_user_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
