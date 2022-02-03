use crate::domain::entities::{Pokemon, PokemonName};
use rustemon::model::pokemon::PokemonSpecies;
use rustemon::blocking::pokemon::pokemon_species;

// ================================= Common Repository Trait ====================================//
pub trait Repository: Send + Sync{
    fn get_pokemon(&self, name: PokemonName) -> Pokemon;
}


// ================================== RustemonRepository  ======================================//

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
        let pokemon = pokemon_species::get_by_name(string_to_static_str(name));
        match pokemon {
            Ok(poke) => pokemon_species_to_pokemon(poke),
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

//================================= Helper Methods ============================================= //

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn pokemon_species_to_pokemon(pokemon: PokemonSpecies) -> Pokemon {

    // Rustemon library has a nested option structure. It is very error prone in such cases. Thus
    // we will try and extract each values individually. We follow the following rules while 
    // unwrapping :
    // 1) For description and habitat if they are not present we replace them with empty strings.

    let name = pokemon.name.unwrap();
    let is_legendary = pokemon.is_legendary.unwrap();
    let mut habitat = "".to_string();
    if pokemon.habitat.is_some() {
        habitat = pokemon.habitat.unwrap().name.unwrap_or("".to_string());
    }

    let mut description = "".to_string();
    if pokemon.flavor_text_entries.is_some() {
        let flavor_text_entries = pokemon.flavor_text_entries.unwrap();
        if flavor_text_entries.len() != 0 {
            description = flavor_text_entries[0].flavor_text.as_ref().unwrap_or(&String::from("")).to_string();
        } 
    }

    Pokemon {
        name,
        description,
        habitat,
        is_legendary,
    }
}

