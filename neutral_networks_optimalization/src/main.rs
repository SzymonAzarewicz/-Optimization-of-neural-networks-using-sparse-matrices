#![allow(dead_code)]



mod structure;

mod matrix;
mod network;
mod activations;


//use eframe::egui;

use activations::SIGMOID;
use network::Network2;

//use structure::Neuron;
//use structure::Layer;
//use structure::Network;



mod visualiser;

fn main() {

    let layers = vec![2, 4, 10, 2]; // Definicja warstw (liczba neuronów w każdej warstwie)
    visualiser::run_visualiser(layers).unwrap(); // Wywołanie funkcji z visualiser.rs
    
    // // Definiujemy rozmiary warstw w sieci: 3 wejścia, 2 warstwy ukryte (4 i 3 neurony) oraz 1 neuron wyjściowy
    // let layer_sizes = [3, 4, 3, 1];

    // // Tworzymy sieć neuronową z zadanymi rozmiarami warstw
    // let mut network = Network::new(&layer_sizes);

    // // Przykładowe dane wejściowe dla sieci (trzy wejścia)
    // let inputs = vec![0.5, -0.3, 0.8];

    // // Propagujemy dane przez całą sieć
    // let outputs = network.forward(inputs);

    // // Wyświetlamy ostateczne wyjście
    // println!("Ostateczne wyjście z sieci: {:?}", outputs);

    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];

    let mut network = Network2::new(vec![2,3,1], 0.5,SIGMOID);


    println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));

    network.train(inputs, targets, 10000);

    println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));


}


