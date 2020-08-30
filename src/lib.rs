#![recursion_limit = "1024"]
extern crate serde_derive;
extern crate serde_json;

mod components;
mod model;
mod repo;
mod time;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<components::App>::new().mount_to_body();
}
