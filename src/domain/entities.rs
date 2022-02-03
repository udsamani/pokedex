use std::convert::TryFrom;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PokemonName(String);

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub description: String, 
    pub habitat: String,
    pub is_legendary: bool,
}



// =========================================== Implementations ==================================//
impl Pokemon {
    pub fn new(name: String, description: String, habitat: String, is_legendary: bool) -> Self {
        Self {
            name,
            description, 
            habitat,
            is_legendary,
        }
    }
}

impl<'a> TryFrom<&'a String> for PokemonName {
    type Error = ();
    fn try_from(name: &'a String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            Err(())
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl From<PokemonName> for String {
    fn from(n: PokemonName) -> Self {
        n.0
    }
}


