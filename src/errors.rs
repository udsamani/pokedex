#[derive(Debug)]
pub enum Error {
    BadRequest,
    Unauthorized,
    TooManyRequests, 
    InternalServerError,
    NotFound, 
    Unknown,
}


pub fn handle_error_code(code: u16) -> Error {
    match code {
        404 => Error::NotFound,
        401 => Error::Unauthorized,
        429 => Error::TooManyRequests,
        400 => Error::BadRequest,
        _ => Error::Unknown,   
    }
}

