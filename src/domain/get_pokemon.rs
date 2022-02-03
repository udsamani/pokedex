use crate::domain::entities::PokemonName;
use std::convert::TryFrom;

pub struct Request<'a> {
    pub name: &'a String,
}

pub struct Response {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}


pub enum Error {
    BadRequest, 
    NotFound, 
    Unknown,
}


pub fn execute(req: Request) -> Result<Response, Error> {
    match PokemonName::try_from(req.name) {
        Ok(name) => Ok(Response{
            name: String::from(name),
            description: String::from("Hi charizard"),
            habitat: String::from("rare"),
            is_legendary: true,
        }),
        _ => Err(Error::BadRequest),
    }
}