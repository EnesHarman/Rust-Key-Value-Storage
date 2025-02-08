use std::io;
use std::path::Path;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};

use kvstorage::{delete_value, get_value, insert_value, update_value, AppState, StorageService};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let file_name = "kv"; //todo Move this to configs
    let path = Path::new(file_name);
    let state = AppState::new(path)
        .await
        .expect("Error while creating app state!");

    let storage_service = web::Data::new(StorageService::new(Arc::clone(&state.kv_storage)));

    HttpServer::new(move || {
        App::new()
            .app_data(storage_service.clone())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/storage")
                            .service(insert_value)
                            .service(get_value)
                            .service(delete_value)
                            .service(update_value)
                    )
            )
            .default_service(
                web::route().to(|| async {
                    actix_web::HttpResponse::NotFound().finish()
                })
            )
    })
        .bind("localhost:8080")?//todo Move this to configs
        .run()
        .await
}