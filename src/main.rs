#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod views;

use actix_files as fs;
use actix_web::{
    delete, get, middleware, post, put, web, App, Error, HttpResponse, HttpServer, Responder,
};
use models::*;
use serde::Deserialize;
use std::env;
use views::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let memos = web::block(move || all_memos(&conn)).await.map_err(|e| {
        log::error!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(IndexView { memos })
}

#[get("/new")]
async fn new_memo() -> impl Responder {
    NewView
}

#[get("/{id}")]
async fn edit_memo(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let memo = web::block(move || find_memo(path.0, &conn))
        .await
        .map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(memo) = memo {
        Ok(EditView { memo })
    } else {
        Err(HttpResponse::NotFound().finish().into())
    }
}

#[derive(Deserialize)]
struct MemoData {
    pub body: String,
}

#[post("/memos")]
async fn create_memo(
    pool: web::Data<DbPool>,
    data: web::Json<MemoData>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || insert_new_memo(&data.body, &conn))
        .await
        .map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::NoContent().finish())
}

#[put("/memos/{id}")]
async fn update_memo(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
    data: web::Json<MemoData>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || update_memo_body(path.0, &data.body, &conn))
        .await
        .map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::NoContent().finish())
}

#[delete("/memos/{id}")]
async fn delete_memo(
    pool: web::Data<DbPool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || remove_memo(path.0, &conn))
        .await
        .map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::NoContent().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let pool = db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(new_memo)
            .service(edit_memo)
            .service(create_memo)
            .service(update_memo)
            .service(delete_memo)
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
