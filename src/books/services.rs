use super::models::{CreatedBookData, UpdateBookData};
use crate::{AppState, BookEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use nanoid::nanoid;

#[get("/books")]
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.book_entries.lock().await.to_vec())
}

#[post("/books")]
async fn create_book(
    data: web::Data<AppState>,
    param_obj: web::Json<CreatedBookData>,
) -> impl Responder {
    let mut book_entries = data.book_entries.lock().await;

    let id = nanoid!(10);

    book_entries.push(BookEntry {
        id,
        name: param_obj.name.clone(),
        publisher: param_obj.publisher.clone(),
    });

    HttpResponse::NoContent()
}

#[put("/books/{id}")]
async fn update_book(
    data: web::Data<AppState>,
    path: web::Path<String>,
    param_obj: web::Json<UpdateBookData>,
) -> impl Responder {
    let id = path.into_inner();
    let mut book_entries = data.book_entries.lock().await;
    for i in 0..book_entries.len() {
        if book_entries[i].id == id {
            book_entries[i].name = param_obj.name.clone();
            book_entries[i].publisher = param_obj.publisher.clone()
        }
    }

    HttpResponse::Ok().json(book_entries.to_vec())
}

#[delete("/books/{id}")]
async fn delete_book(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut book_entries = data.book_entries.lock().await;
    *book_entries = book_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(book_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books)
        .service(create_book)
        .service(update_book)
        .service(delete_book);
}
