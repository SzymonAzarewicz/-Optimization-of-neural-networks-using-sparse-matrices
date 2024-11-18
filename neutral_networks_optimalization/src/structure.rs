#![allow(dead_code)]


use rand::Rng;
use std::io;


pub struct Neuron{
    weights: Vec<f64>,
    bias: f64,
    output: f64,
}
pub struct Layer{
    neurons: Vec<Neuron>,
}
pub struct Network {
    layers: Vec<Layer>,
}

impl Neuron{
    //inicjalizacja
    pub fn new(num_inputs: usize) -> Self{
        let mut rng = rand::thread_rng(); // losowy gen
        let mut weights = Vec::new();
        let mut bias = 0.0;
        let output = 0.0;
    //uzupełnienie
    for _ in 0..num_inputs {
            weights.push(rng.gen_range(-1.0..1.0));
            bias = rng.gen_range(-1.0..1.0);
        }
    //zwracanie
        Neuron { weights, bias, output}
    }

    //obliczanie wyjścia 
    pub fn forward(&mut self, inputs: &Vec<f64>){
        let mut sum = 0.0;
        for (w, i) in self.weights.iter().zip(inputs) {
            sum += w * i;
            //self.output = get_activation_function(sum);
        }
        sum = sum + self.bias;
        self.output = get_activation_function(sum);
    }

     pub fn display(&self) {
         println!("Neuron:");
         println!("  Weights: {:?}", self.weights);
         println!("  Bias: {}", self.bias);
         println!("  Output: {}", self.output);
     }
}

impl Layer {
    pub fn new(neurons_count: usize, inputs_count: usize) -> Self {
        let mut neurons = Vec::with_capacity(neurons_count);
        for _ in 0..neurons_count {
            neurons.push(Neuron::new(inputs_count));
        }
        Layer { neurons }
    }

    pub fn forward(&mut self, inputs: &Vec<f64>)-> Vec<f64> {
        let mut outputs = Vec::new();
        for neuron in self.neurons.iter_mut() {
            neuron.forward(inputs);
            outputs.push(neuron.output);
        }
        outputs
    }
    pub fn display(&self) {
        for (i, neuron) in self.neurons.iter().enumerate() {
            println!("Neuron {}:", i + 1);
            neuron.display();
        }
    }
    
}

impl Network{
    pub fn new(layer_sizes: &[usize]) -> Self {
        let mut layers = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i + 1], layer_sizes[i]));
        }

        Self { layers }
    }

    pub fn forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        let mut activations = inputs;
        for layer in self.layers.iter_mut() {
            activations = layer.forward(&activations);
        }
        activations
    }
    
    
}

pub fn get_activation_function(sum: f64) ->f64{
    println!("Choose activation function:");
    println!("1. ReLU");
    println!("2. Sigmoid");
    println!("3. Tanh");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim(){
        "1" => relu(sum),
        "2" => sigmoid(sum),
        "3" => tanh(sum),
        _ => {
            println!("Nieprawidłowy wybór. Domyślnie wybrano sigmoid");
            sigmoid(sum)
        }
    } 
}
pub fn relu(i: f64) -> f64 {
    i.max(0.0)
}
pub fn sigmoid(i: f64) -> f64 {
    1.0 / (1.0 + (-i).exp())
}
pub fn tanh(i: f64) -> f64 {
    i.tanh()
}




