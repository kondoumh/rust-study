use photon_rs::{filters::filter, PhotonImage};
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

const FILTERS: &[&str] = &[
    "oceanic",
    "islands",
    "marine",
    "seagreen",
    "flagblue",
    "liquid",
    "diamante",
    "radio",
    "twenties",
    "rosetint",
    "mauve",
    "bluechrome",
    "vintage",
    "perfume",
    "serenity",
];

#[wasm_bindgen]
pub fn apply_some_filter(mut img: &mut PhotonImage) -> String {
    let mut rng = rand::thread_rng();
    let chosen = FILTERS
        .choose(&mut rng)
        .expect("Can't choose a filter name!");
    filter(&mut img, chosen);
    chosen.to_string()
}
