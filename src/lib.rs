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
    yew::start_app::<components::App>();
}
