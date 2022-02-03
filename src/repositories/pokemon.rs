use crate::domain::entities::{Pokemon, PokemonName};
use rustemon::model::pokemon::PokemonSpecies;
use rustemon::blocking::pokemon::pokemon_species;
use crate::errors::{Error, handle_error_code};
use serde::Deserialize;
use ureq;

const CAVE: &str = "cave";

// ================================= Common Repository Trait ====================================//
pub trait Repository: Send + Sync{
    fn get_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error>;
    fn translate_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error>;
}


// ================================== RustemonRepository  ======================================//

pub struct RustemonRepository {
    yoda: String,
    shakespeare: String,
}


impl RustemonRepository {
    pub fn new() -> Self {
        Self {
            yoda: String::from("https://api.funtranslations.com/translate/yoda.json"),
            shakespeare: String::from("https://api.funtranslations.com/translate/shakespeare.json"), 
        }
    }

    pub fn get_pokemon_details(&self, name: String) -> Result<Pokemon, Error> {
        // Implemented a hack here. The rustemon library expects &'static str. The only way to do this here
        // is by leaking memory. Ideally need to raise a PR for rustemon library asking to fix that. 
        let pokemon = pokemon_species::get_by_name(string_to_static_str(name));
        match pokemon {
            Ok(poke) => pokemon_species_to_pokemon(poke),
            Err(e) => {
                // Due to a bug in Rustemon, not proper codes are being propogated
                // As a result we are sending in 500 error.
                return Err(Error::InternalServerError)
            }
        }
    }
}


impl Repository for RustemonRepository {
    fn get_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error> {
        self.get_pokemon_details(String::from(name))
    }

    fn translate_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error> {
        // 1) Fetch the pokemon first.
        let mut pokemon = self.get_pokemon_details(String::from(name)).unwrap();

        // 2) Call Yoda or Shakespeare translator endpoint accorrdingly.
        let mut translator_url = &self.shakespeare;
        if pokemon.habitat == CAVE || pokemon.is_legendary {
            translator_url = &self.yoda;
        }

        let url = format!("{}?text={}", *translator_url, pokemon.description);
        let res = match ureq::get(&url)
            .set("X-FunTranslations-Api-Secret", "ipSacTXQqq9wWGtaTOxREweF")
            .call() 
        {
            Ok(res) => {
                res
            },
            Err(ureq::Error::Status(u, _)) =>  {
                return Err(handle_error_code(u));
            },
            _ => return Err(Error::InternalServerError)
        };
        let json = match res.into_json::<TranslationJson>() {
            Ok(json) => Ok(json),
            _ => Err(()) ,
        };

        //3) Update existing pokemon with translated description
        pokemon.description = json.unwrap().contents.translated;
        Ok(pokemon) 
    }
}

#[derive(Deserialize, Debug)]
struct TranslationJson {
    success: TranslationSuccess,
    contents: TranslationContent,
}

#[derive(Deserialize, Debug)]
struct TranslationSuccess {
    total: u32,
}

#[derive(Deserialize, Debug)]
struct TranslationContent {
    translated: String,
    text: String,
    translation: String,
}

//================================= Helper Methods ============================================= //

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn pokemon_species_to_pokemon(pokemon: PokemonSpecies) -> Result<Pokemon, Error> {

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
            description = flavor_text_entries[0]
                .flavor_text
                .as_ref()
                .unwrap_or(&String::from("")).to_string();
        } 
    }
    description = description.replace('\n', " ");

    Ok(Pokemon {
        name,
        description,
        habitat,
        is_legendary,
    })
}

