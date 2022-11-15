use crate::models::note_models::{Note, NoteMessage};
use crate::api_error::ApiError;
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/")]
async fn find_all() -> Result<HttpResponse, ApiError> {
  let notes = Note::find_all()?;
    Ok(HttpResponse::Ok().json(notes))
}

#[get("/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
  let note = Note::find(id.into_inner())?;
  Ok(HttpResponse::Ok().json(note))
}

#[post("/")]
async fn create(note: web::Json<NoteMessage>) -> Result<HttpResponse, ApiError> {
  let note = Note::create(note.into_inner())?;
  Ok(HttpResponse::Ok().json(note))
}

#[put("/{id}")]
async fn update(id: web::Path<Uuid>, note: web::Json<NoteMessage>) -> Result<HttpResponse, ApiError> {
  let note = Note::update(id.into_inner(), note.into_inner())?;
  Ok(HttpResponse::Ok().json(note))
}

#[delete("/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
  let note = Note::delete(id.into_inner())?;
  Ok(HttpResponse::Ok().json(json!({"Note deleted": note})))
}