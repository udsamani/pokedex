# Pokedex Server

The Pokedex server is written in Rust. In order for this service to work one will need to install Rust and enable the nightly version. You can enable nightly version by running : `rustup default nightly`. 

Once done the shell would be configured to use RUST's nightly version.

## Running the `Pokedex` Server

### Using `Docker` Image
The Docker image used as base image is the `debian:buster-slim`.  From the root directory of the project build the docker image using the following command.

`docker build -t udsamani/pokedex .`

### Running Locally
If you prefer running the application directly without using Docker, do the following from the project root directory:

`cargo run`

You server will be started on port 8080.


# Design

The application is desgined based on `Hexagonal` architecture system. The idea is to make the core of the application independent from the dependencies. The core is often called the domain, it is where all the business rules and entities of you are application are found. The dependencies are basically the rest of your application: databases, frameworks, libraries, messages queues etc. In this essance, this architectuure is a way to separate the business part of your application to the implementation details.

There are serveral advantages to this architecture: 
- You can change the domain without changing the dependencies
- You can change the dependencies withoyt changing the domain
- You can easily test the domain


With the above architecture description in mind, pokedex is designed in the following way:

### API Layer 
All the APIs are exposed from this folder. Each rust file in the `src/api/` folder corresponds to the API we want to call. 

### Domain Layer
All the requests from the API Layer are passed on to the domain layer. Thus for each endpoint as in `src/api` we will have a corresponding file in `src/domain`. There are some other files present in the domain folder. These correspond to the business entities. There is a great advantage of designing the pokedex application in this manner. For example, let's say that tomorrow the client comes and says that they want a CLI based tool for fetching the pokemon details. Now the only thing which we would need to do is come up wth a `CLI Layer`, which would be reponsible for handling CLI logic and the business logic would still be abstracted out in the domain layer.

### Repository Layer
Repository layer represents dependencies like databases, framework etc. With our current business specification, we are fetching the `Pokemon` data from the API's provided by `https://pokeapi.co/` . We have come up with a standart trait defined as follows.

```rust
pub trait Repository: Send + Sync{
    fn get_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error>;
    fn translate_pokemon(&self, name: PokemonName) -> Result<Pokemon, Error>;
}
```
This helps us in abstacting out or dependencies. Let's say tomorrow instead of using `https://pokeapi.co/` we start talking to a database to obtain pokemon details. We can then do this by just implementing this trait for some `DatabaseRepository` struct. Thus allowing our application to work in two different ways. Moreover, this even makes reverting easier. 

