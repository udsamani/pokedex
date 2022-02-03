use crate::domain::entities::{Pokemon, PokemonName};
use rustemon;

pub trait Repository: Send + Sync{
    fn get_pokemon(&self, name: PokemonName) -> Pokemon;
}



pub struct RustemonRepository {
    yoda_translator_url: String,
    shakespeare_translator_url: String,
}


impl RustemonRepository {
    pub fn new() -> Self {
        Self {
            yoda_translator_url: String::from("yoda"),
            shakespeare_translator_url: String::from("shakespeare"), 
        }
    }

    pub fn get_pokemon_details(&self, name: String) -> Pokemon {
        // Implemented a hack here. The rustemon library expects &'static str. The only way to do this here
        // is by leaking memory. Ideally need to raise a PR for rustemon library asking to fix that. 
        let pokemon = rustemon::blocking::pokemon::pokemon_species::get_by_name(string_to_static_str(name));
        match pokemon {
            Ok(poke) => Pokemon {
                name: poke.name.unwrap(),
                description: String::from("Figuring out"),
                habitat: poke.habitat.unwrap().name.unwrap(),
                is_legendary: poke.is_legendary.unwrap(),
            },
            _ => Pokemon {
                name: String::from("Udit"),
                description: String::from("Figuring out"),
                habitat: String::from("Human"),
                is_legendary: true,
            } 
        }
    }
}


impl Repository for RustemonRepository {
    fn get_pokemon(&self, name: PokemonName) -> Pokemon {
        self.get_pokemon_details(String::from(name))
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

