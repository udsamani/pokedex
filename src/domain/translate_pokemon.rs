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

//==================================== Tests ====================================================//
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::PokemonName;
    use crate::repositories::pokemon::RustemonRepository;

    #[test]
    fn bad_request_error() {
        let repo = Arc::new(RustemonRepository::new());
        let request = Request {
            name: &String::from("")
        };

        let response = execute(request, repo);

        match response {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn internal_server_error() {
        let repo = Arc::new(RustemonRepository::new());
        let request = Request {
            name: &String::from("udit")
        };

        let response = execute(request, repo);

        match response {
            Err(Error::InternalServerError) => {}
            _ => unreachable!(),
        }; 
    }

    #[test]
    fn success_yoda_translation() {
        let repo = Arc::new(RustemonRepository::new());
        let successful_translation = String::from("Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.");
        let request = Request {
            name: &String::from("mewtwo")
        };

        let response = execute(request, repo);
        match response {
            Ok(resp) => {
                assert_eq!(resp.description, successful_translation);
            },
            _ => unreachable!(),
        };
    }

    #[test]
    fn failure_shakespeare_translation() {
        let repo = Arc::new(RustemonRepository::new());
        let successful_translation = String::from("Spits fire yond is hot enow to melt boulders.known to cause forest fires unintentionally.");
        let request = Request {
            name: &String::from("charizard")
        };

        let response = execute(request, repo);
        match response {
            Ok(resp) => {
                assert_eq!(resp.description, successful_translation);
            },
            _ => unreachable!(),
        };
    }

    
}