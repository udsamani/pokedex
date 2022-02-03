mod api;
mod domain;

#[macro_use]
extern crate rouille;

fn main() {
    api::server("localhost:8000")
}