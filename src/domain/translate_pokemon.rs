use crate::domain::entities::PokemonName;
use std::convert::TryFrom;
use crate::errors::Error;
use crate::repositories::pokemon::Repository;
use std::sync::Arc;

// Request for /pokemon/translate/{name} endpoint
pub struct Request<'a> {
    pub name: &'a String,
}

// Respone for /pokemon/translate/{name} endpoint
pub struct Response {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

//==================================== Request Exectuion ========================================//
pub fn execute(req: Request, repo: Arc<dyn Repository>) -> Result<Response, Error> {
    match PokemonName::try_from(req.name) {
        Ok(name) => {
            match repo.translate_pokemon(name) {
                Ok(pokemon) => {
                    Ok(Response{
                        name: pokemon.name,
                        description: pokemon.description,
                        habitat: pokemon.habitat,
                        is_legendary: pokemon.is_legendary,})
                },
                Err(e) => return Err(e)
            }
            
        },
        _ => Err(Error::BadRequest),
    }
}
