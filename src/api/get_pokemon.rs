use crate::api::Status;
use crate::domain::get_pokemon;
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
    let request = get_pokemon::Request { name };
    match get_pokemon::execute(request, repo) {
        Ok(get_pokemon::Response {
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
        Err(get_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(get_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
        Err(get_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}