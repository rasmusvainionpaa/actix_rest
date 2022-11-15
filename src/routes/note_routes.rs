use actix_web::web;
use crate::controllers::note_controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/api/v1/notes")
      .service(note_controller::find_all)
      .service(note_controller::find)
      .service(note_controller::create)
      .service(note_controller::update)
      .service(note_controller::delete)
  );
}