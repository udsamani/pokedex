use crate::api::Status;
use crate::domain::translate_pokemon;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;
use rouille;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}


pub fn serve(name: &String, repo: Arc<dyn Repository>) -> rouille::Response {
    let request = translate_pokemon::Request { name };
    match translate_pokemon::execute(request, repo) {
        Ok(translate_pokemon::Response {
            name, 
            description,
            habitat,
            is_legendary
        }) => rouille::Response::json(&Response {
            name, 
            description,
            habitat,
            is_legendary
        }),
        Err(translate_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(translate_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(translate_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}