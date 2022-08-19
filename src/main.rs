use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(handlers::get_home))
            .route("/login", web::post().to(handlers::login_user))
            .route("/register", web::post().to(handlers::register_user))
            .route("/logout", web::post().to(handlers::logout_user))
            .route("/task", web::post().to(handlers::add_task))
            .route("/tasks", web::get().to(handlers::get_tasks))
            .route("/task/{id}", web::get().to(handlers::get_task_by_id))
            .route("/task", web::delete().to(handlers::delete_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}