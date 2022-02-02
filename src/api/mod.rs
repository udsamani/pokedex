mod fetch_pokemon;
mod translate_pokemon;


pub fn server(url: &str) {
    rouille::start_server(url, move |req| {
        router!(req, 
            (GET) (/pokemon/{name: String}) => {
                fetch_pokemon::serve(name)
            },
            (GET) (/pokemon/translate/{name: String}) => {
                translate_pokemon::serve(name)
            },
            _ => rouille::Response::from(Status::NotFound)
        )
    });
}


enum Status {
    Ok, 
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}


impl From<Status> for rouille:Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::InternalServerError => 500,
        };

        Self {
            status_code,
            headers: vec![],
            data: rouille:ResponseBody::empty(),
            upgrade: None,
        }
    }
}