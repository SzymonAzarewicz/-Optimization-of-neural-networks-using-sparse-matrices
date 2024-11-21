//source https://github.com/mathletedev
//General Public License (GPL) wersja 3

//use std::f64::consts::E;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Activation {
    pub name: String, // Tylko nazwa funkcji aktywacji
}

impl Activation {
    pub fn function(&self) -> &'static dyn Fn(f64) -> f64 {
        match self.name.as_str() {
            "sigmoid" => &|x: f64| 1.0 / (1.0 + std::f64::consts::E.powf(-x)),
            _ => panic!("Unknown activation function: {}", self.name),
        }
    }

    pub fn derivative(&self) -> &'static dyn Fn(f64) -> f64 {
        match self.name.as_str() {
            "sigmoid" => &|x: f64| x * (1.0 - x),
            _ => panic!("Unknown activation function: {}", self.name),
        }
    }

    pub fn new(name: &str) -> Result<Self, String> {
        match name {
            "sigmoid" => Ok(Activation { name: name.to_string() }),
            _ => Err(format!("Unknown activation function: {}", name)),
        }
    }
}
