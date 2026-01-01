# What The Particle

This project is comprised of two main components:

1. Mount Charles, a custom GEANT4-style particle propagator.
2. What The Particle, an interactive guessing game which uses Mount Charles.

This project was originally created as an outreach tool for the LiquidO experiment, and was partly inspired by the Zooniverse's [Name That Neutrino][https://www.zooniverse.org/projects/icecubeobservatory/name-that-neutrino] project.

Mount Charles is written in pure Rust, while the game renders the results of the simulation using JavaScript (specifically ThreeJS).

## Mount Charles

## Development

### Compiling to JS
In order to expose an API usable from JS, the `wasm_bindgen` crate is used. In the top-level `lib.rs` file, a `WASMWorld` struct is created, annotated with `#[wasm_bindgen]`. The `WASMWorld` struct can then be `impl`-ed , adding any API that is desired.

In order to compile this, run (in the root directory of the project, next to `Cargo.toml`):
``` zsh
wasm-pack build --target web -m no-install
```
(note that the installed `wasm-bindgen` and the cargo `wasm-bindgen` versions must match exactly).

