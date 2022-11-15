use crate::api_error::ApiError;
use crate::database;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schemas::note_schema::note;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "note"]
pub struct Note {
  pub id: Uuid,
  pub title: String,
  pub body: String,
  #[serde(skip_serializing)]
  pub created_at: NaiveDateTime,
  pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "note"]
pub struct NoteMessage {
  pub title: String,
  pub body: String,
}

impl From<NoteMessage> for Note {
  fn from(note: NoteMessage) -> Self {
    Note {
      id: Uuid::new_v4(),
      title: note.title,
      body: note.body,
      created_at: Utc::now().naive_utc(),
      updated_at: None,
    }
  }
}

impl Note {
  pub fn find_all() -> Result<Vec<Note>, ApiError> {
    let conn = database::connection()?;
    let query = note::table.into_boxed();
    let notes = query.load::<Note>(&conn)?;
    Ok(notes)
  }

  pub fn find(id: Uuid) -> Result<Note, ApiError> {
    let conn = database::connection()?;
    let note = note::table.find(id).first::<Note>(&conn)?;
    Ok(note)
  }

  pub fn create(params: NoteMessage) -> Result<Note, ApiError> {
    let conn = database::connection()?;
    let note = Note::from(params);

    let note = diesel::insert_into(note::table)
      .values(&note)
      .get_result(&conn)?;
    Ok(note)
  }

  pub fn update(id: Uuid, updated_note: NoteMessage) -> Result<Note, ApiError> {
    let conn = database::connection()?;
    let note = diesel::update(note::table)
      .filter(note::id.eq(id))
      .set(updated_note)
      .get_result(&conn)?;
    Ok(note)
  }

  pub fn delete(id: Uuid) -> Result<usize, ApiError> {
    let conn = database::connection()?;
    let result = diesel::delete(note::table.find(id)).execute(&conn)?;
    Ok(result)
  }
}