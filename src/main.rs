mod api;
mod domain;
mod repositories;
mod errors;
use std::sync::Arc;

#[macro_use]
extern crate rouille;

fn main() {
    let repo = Arc::new(repositories::pokemon::RustemonRepository::new());
    println!("Starting server on 8080 port");
    api::server("localhost:8080", repo)
}