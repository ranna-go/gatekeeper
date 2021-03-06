use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};
use providers::{provider::Provider, token::BasicTokenPool};

mod providers;
mod routes;

pub struct Context {
    basic_tokens: Arc<dyn Provider>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let bind_address = std::env::var("BINDADDRESS").unwrap_or("0.0.0.0:80".into());

    let basic_tokens: Arc<dyn Provider> = Arc::new(BasicTokenPool::new()?);

    HttpServer::new(move || {
        App::new()
            .data(Context {
                basic_tokens: Arc::clone(&basic_tokens),
            })
            .wrap(middleware::Logger::default())
            .route(
                "/auth/basic_token",
                web::get().to(routes::token_auth::handle),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
