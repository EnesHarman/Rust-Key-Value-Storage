use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use serde::Deserialize;
use crate::{StorageError, StorageService};

#[post("/add")]
pub async fn insert_value(storage_service: web::Data<StorageService>, element: web::Json<KVElement>) -> Result<HttpResponse> {
    let element = element.into_inner();
    let value = storage_service.insert(element.key, element.value).await?;
    Ok(HttpResponse::Ok().body(value))
}

#[get("/get")]
pub async fn get_value(storage_service: web::Data<StorageService>, param: web::Query<Key>) -> Result<HttpResponse, StorageError> {
    let key = param.into_inner().key;
    let value = storage_service.get(key).await?;
    Ok(HttpResponse::Ok().body(value))
}

#[delete("/delete")]
pub async fn delete_value(storage_service: web::Data<StorageService>, param: web::Query<Key>) -> Result<HttpResponse, StorageError> {
    let key = param.into_inner().key;
    let value = storage_service.delete(key).await?;
    Ok(HttpResponse::Ok().body(value))
}

#[put("/update")]
pub async fn update_value(storage_service: web::Data<StorageService>, body: web::Json<KVElement>) -> Result<HttpResponse> {
    let body = body.into_inner();
    let value = storage_service.update(body.key, body.value).await?;
    Ok(HttpResponse::Ok().body(value))
}

#[derive(Deserialize)]
pub struct KVElement {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct Key {
    key: String,
}


