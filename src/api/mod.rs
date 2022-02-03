mod get_pokemon;
mod translate_pokemon;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

pub fn server(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req, 
            (GET) (/pokemon/{name: String}) => {
                get_pokemon::serve(&name, repo.clone())
            },
            (GET) (/pokemon/translate/{name: String}) => {
                translate_pokemon::serve(&name, repo.clone())
            },
            _ => rouille::Response::from(Status::NotFound)
        )
    });
}

enum Status {
    Ok, 
    BadRequest,
    Unauthorized,
    TooManyRequests,
    NotFound,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Unauthorized => 401,
            Status::TooManyRequests => 429,
            Status::InternalServerError => 500,
        };

        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}