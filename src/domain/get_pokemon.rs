use crate::domain::entities::PokemonName;
use std::convert::TryFrom;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

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


pub fn execute(req: Request, repo: Arc<dyn Repository>) -> Result<Response, Error> {
    match PokemonName::try_from(req.name) {
        Ok(name) => {
            let pokemon = repo.get_pokemon(name);
            Ok(Response{
            name: pokemon.name,
            description: pokemon.description,
            habitat: pokemon.habitat,
            is_legendary: pokemon.is_legendary,})
        },
        _ => Err(Error::BadRequest),
    }
}