//source https://github.com/mathletedev
//General Public License (GPL) wersja 3

use std::f64::consts::E;

use serde::{Serialize};
use rand::Rng;
use std::fs;

#[derive(Clone, Serialize)]
pub struct Activation<'a> {
    #[serde(skip)] // Pomijamy serializację funkcji
    pub function: &'a dyn Fn(f64) -> f64,
    #[serde(skip)] // Pomijamy serializację funkcji
    pub derivative: &'a dyn Fn(f64) -> f64,
    pub name: &'a str, // Nazwa funkcji aktywacji
}

impl<'a> Activation<'a> {
    pub fn from_name(name: &'a str) -> Self {
        match name {
            "sigmoid" => SIGMOID,
            _ => panic!("Unknown activation function: {}", name),
        }
    }
}

pub const SIGMOID: Activation = Activation {
    function: &|x| 1.0 / (1.0 + std::f64::consts::E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
    name: "sigmoid",
};