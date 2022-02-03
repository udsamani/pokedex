mod api;
mod domain;
mod repositories;
use std::sync::Arc;

#[macro_use]
extern crate rouille;

fn main() {
    let repo = Arc::new(repositories::pokemon::RustemonRepository::new());
    api::server("localhost:8000", repo)
}