use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger;

mod graphql;
mod signup;
mod login;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(graphql::GraphQl::new())
            .service(login::login)
            .service(signup::signup)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
