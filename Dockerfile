#1) Use default dart image as the build image
FROM rust AS builder 

#2) Copy the current folder into the build folder
COPY . /app

#3) Set the work directory
WORKDIR /app

#4) Build the application
RUN cargo build --release

#5) Use slim alpine image
FROM alpine:3.14

#6) Copy the runtime files
COPY --from=builder /app/target/release/pokedex /app/pokedex
WORKDIR /app



#7) Start the server
CMD ["./pokedex"]
