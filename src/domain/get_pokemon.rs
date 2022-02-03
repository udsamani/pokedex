use crate::domain::entities::PokemonName;
use std::convert::TryFrom;
use crate::repositories::pokemon::Repository;
use crate::errors::Error;
use std::sync::Arc;

// Request for /pokemon/{name} endpoint.
pub struct Request<'a> {
    pub name: &'a String,
}

// Response for /pokemon/{name} endpoint.
pub struct Response {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}


//==================================== Request Exectuion ========================================//

// execute function mainly acts as handler. Here it acts as a handler for the endpoint
// /pokemon/{name}.
pub fn execute(req: Request, repo: Arc<dyn Repository>) -> Result<Response, Error> {
    match PokemonName::try_from(req.name) {
        Ok(name) => {
            match repo.get_pokemon(name) {
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
    fn successful_pokemon_found_1() {
        let repo = Arc::new(RustemonRepository::new());
        let request = Request {
            name: &String::from("charizard")
        };

        let response = execute(request, repo);
        match response {
            Ok(pokemon) => {
                assert_eq!("charizard", pokemon.name);
            }
            _ => unreachable!(),
        }; 
    }

    #[test]
    fn successful_pokemon_found_2() {
        let repo = Arc::new(RustemonRepository::new());
        let request = Request {
            name: &String::from("mewtwo")
        };

        let response = execute(request, repo);
        match response {
            Ok(pokemon) => {
                assert_eq!("mewtwo", pokemon.name);
            }
            _ => unreachable!(),
        }; 
    }

    #[test]
    fn successful_pokemon_found_3() {
        let repo = Arc::new(RustemonRepository::new());
        let request = Request {
            name: &String::from("wormadam")
        };

        let response = execute(request, repo);
        match response {
            Ok(pokemon) => {
                assert_eq!("wormadam", pokemon.name);
            }
            _ => unreachable!(),
        }; 
    }
}