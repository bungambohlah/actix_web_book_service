use actix_web::{get, middleware, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

mod books;
use books::services;

struct AppState {
    book_entries: Mutex<Vec<BookEntry>>,
}
#[derive(Serialize, Deserialize, Clone)]
struct BookEntry {
    id: String,
    name: String,
    publisher: String,
}

#[get("/")]
async fn index() -> String {
    "This is a book route".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        book_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
