mod api;
mod domain;
mod repositories;
mod errors;
use std::sync::Arc;
use log::info;
use std::env;
use env_logger;

#[macro_use]
extern crate rouille;

fn main() {
    env_logger::init();
    let repo = Arc::new(repositories::pokemon::RustemonRepository::new());
    info!("Starting server on 8080 port");
    api::server("0.0.0.0:8080", repo)
}