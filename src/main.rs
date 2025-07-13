use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files as fs;
use tera::Tera;
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    println!("🦀 Starting Rust + Actix Web server for devforge.in");
    println!("🌐 Server will be available at: http://127.0.0.1:8081");
    
    // Initialize Tera template engine
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .wrap(Logger::default())
            // Static files
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // Web routes
            .service(
                web::scope("")
                    .route("/", web::get().to(handlers::web::index))
                    .route("/about", web::get().to(handlers::web::about))
                    .route("/contact", web::get().to(handlers::web::contact))
                    .route("/contact", web::post().to(handlers::web::contact_post))
            )
            // API routes
            .service(
                web::scope("/api")
                    .route("/info", web::get().to(handlers::api::info))
                    .route("/contacts", web::get().to(handlers::api::contacts))
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
