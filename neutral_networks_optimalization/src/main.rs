#![allow(dead_code)]



mod structure;

mod matrix;
mod network;
mod activations;

use crate::activations::Activation;

//use eframe::egui;

//use activations::SIGMOID;
use network::Network2;

//use structure::Neuron;
//use structure::Layer;
//use structure::Network;



mod visualiser;

fn main() {

     let layers = vec![2, 3, 1];
     let inputs = vec![
         vec![0.0, 0.0], // (0 AND 0) -> 0
         vec![0.0, 1.0], // (0 AND 1) -> 0
         vec![1.0, 0.0], // (1 AND 0) -> 0
         vec![1.0, 1.0], // (1 AND 1) -> 1
     ];
     let targets = vec![
         vec![0.0], // Oczekiwana wartość wyjściowa
         vec![0.0],
         vec![0.0],
         vec![1.0],
     ];
 
     // 1. Stworzenie sieci i trenowanie od zera
     let mut network = Network2::new(layers.clone(), 0.01, Activation::new("sigmoid").unwrap());
     println!("Przed treningiem (sieć początkowa):");
     for (i, input) in inputs.iter().enumerate() {
         let output = network.feed_forward(input.clone());
         println!("Input: {:?}, Expected: {:?}, Predicted: {:?}", input, targets[i], output);
     }
 
     // Trening sieci
     network.train(inputs.clone(), targets.clone(), 10000);
     println!("\nPo pierwszym treningu:");
     for (i, input) in inputs.iter().enumerate() {
         let output = network.feed_forward(input.clone());
         println!("Input: {:?}, Expected: {:?}, Predicted: {:?}", input, targets[i], output);
     }
 
     // Zapisanie sieci do pliku
     if let Err(e) = network.save("network.json") {
         eprintln!("Failed to save network: {}", e);
         return;
     } else {
         println!("\nSieć została zapisana do pliku network.json.");
     }
 
     // 2. Wczytanie sieci z pliku
     let mut loaded_network = match Network2::load("network.json") {
         Ok(network) => network,
         Err(e) => {
             eprintln!("Failed to load network: {}", e);
             return;
         }
     };
     println!("\nSieć została wczytana i gotowa do dalszego trenowania.");
 
     // 3. Kontynuacja treningu
     loaded_network.train(inputs.clone(), targets.clone(), 5000);
     println!("\nPo dalszym trenowaniu:");
     for (i, input) in inputs.iter().enumerate() {
         let output = loaded_network.feed_forward(input.clone());
         println!("Input: {:?}, Expected: {:?}, Predicted: {:?}", input, targets[i], output);
     }
    
    

    //WIZUALIZACJA - DZIAŁA
    //visualiser::run_visualiser(layers).unwrap(); // Wywołanie funkcji z visualiser.rs
    
}


