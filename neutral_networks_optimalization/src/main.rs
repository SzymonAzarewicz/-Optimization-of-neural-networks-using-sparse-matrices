mod structure;


//use structure::Neuron;
//use structure::Layer;
use structure::Network;

fn main() {

    // Definiujemy rozmiary warstw w sieci: 3 wejścia, 2 warstwy ukryte (4 i 3 neurony) oraz 1 neuron wyjściowy
    let layer_sizes = [3, 4, 3, 1];

    // Tworzymy sieć neuronową z zadanymi rozmiarami warstw
    let mut network = Network::new(&layer_sizes);

    // Przykładowe dane wejściowe dla sieci (trzy wejścia)
    let inputs = vec![0.5, -0.3, 0.8];

    // Propagujemy dane przez całą sieć
    let outputs = network.forward(inputs);

    // Wyświetlamy ostateczne wyjście
    println!("Ostateczne wyjście z sieci: {:?}", outputs);
}

