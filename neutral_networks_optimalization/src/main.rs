mod structure;
use structure::Neuron;
use structure::Layer;


fn main() {

// Tworzymy warstwę z 3 neuronami, każdy neuron ma 2 wejścia
    let mut layer = Layer::new(3, 2);

// Przykładowe dane wejściowe
    let inputs = vec![0.5, -1.5];

// Obliczamy output dla każdego neuronu w warstwie
    layer.forward(&inputs);

// Wyświetlamy zawartość wszystkich neuronów w warstwie
    layer.display();

}
