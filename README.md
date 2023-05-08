# libstark-bindgen-test

This repo is a binding for the BairWitness (https://github.com/elibensasson/libSTARK/blob/master/libstark/src/languages/Bair/BairWitness.hpp) object from C++'s library libSTARK into Rust. It serializes the object in c++ and returns it into rust, which deserializes and implements the necessary methods.
 
## Running tests

For running the code you need libSTARK cloned and compiled at the same parent folder as this one, like shown below

├── libSTARK

├── libstark-bindgen-test

Once this is the case you can just run `cargo build` and `cargo test`. Rust's build process should take care of the library linking process
