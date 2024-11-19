// Created using ChatGPT
use eframe::egui;

pub fn run_visualiser(layers: Vec<usize>) -> Result<(), eframe::Error> {
    // Obliczenie dynamicznego rozmiaru okna na podstawie liczby warstw i neuronów
    let total_layers = layers.len();
    let max_neurons = *layers.iter().max().unwrap_or(&1);

    let window_width = (total_layers as f32 * 150.0).clamp(600.0, 1600.0); // Szerokość okna
    let window_height = (max_neurons as f32 * 100.0).clamp(400.0, 1200.0); // Wysokość okna

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(window_width, window_height)), // Dynamicznie ustawiony rozmiar
        ..Default::default()
    };

    eframe::run_native(
        "Neural Network Visualization",
        options,
        Box::new(move |_cc| Box::new(MyApp::new(layers))),
    )
}

struct MyApp {
    layers: Vec<usize>, // Liczba neuronów w każdej warstwie
}

impl MyApp {
    pub fn new(layers: Vec<usize>) -> Self {
        Self { layers }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let available_size = ui.available_size(); // Dostępna przestrzeń

                // Dynamiczne obliczanie rozmiaru neuronów
                let neuron_size = (available_size.y / (*self.layers.iter().max().unwrap_or(&1) as f32 + 1.5))
                    .min(available_size.x / (self.layers.len() as f32 + 2.5))
                    .clamp(5.0, 50.0);

                let vertical_spacing = neuron_size * 1.5; // Odstęp między neuronami w pionie
                let horizontal_spacing = neuron_size * 2.0; // Odstęp między warstwami

                let mut layer_positions: Vec<Vec<egui::Pos2>> = vec![]; // Pozycje neuronów

                ui.horizontal(|ui| {
                    for (layer_index, &neurons) in self.layers.iter().enumerate() {
                        let mut current_layer_positions = vec![];
                
                        ui.vertical(|ui| {
                            // Wyśrodkowanie neuronów w pionie
                            let vertical_offset = (available_size.y - neurons as f32 * neuron_size
                                - (neurons as f32 - 1.0) * vertical_spacing)
                                .max(0.0) / 2.0;
                            ui.add_space(vertical_offset);
                
                            for _ in 0..neurons {
                                ui.add_space(vertical_spacing / 2.0); // Odstęp przed neuronem
                                let rect = ui.allocate_ui_with_layout(
                                    egui::vec2(neuron_size, neuron_size),
                                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                    |ui| {
                                        ui.painter().circle_filled(
                                            ui.max_rect().center(),
                                            neuron_size / 2.0,
                                            layer_color(layer_index, self.layers.len()),
                                        );
                                    },
                                ).response.rect;
                
                                current_layer_positions.push(rect.center());
                                ui.add_space(vertical_spacing / 2.0); // Odstęp po neuronie
                            }
                        });
                
                        // Dodanie odstępu między warstwami neuronów
                        layer_positions.push(current_layer_positions);
                        ui.add_space(horizontal_spacing);
                
                        // Rysowanie połączeń między warstwami
                        if let Some(prev_layer_positions) = layer_positions.get(layer_index.wrapping_sub(1)) {
                            let painter = ui.painter();
                            for &prev_pos in prev_layer_positions {
                                for &curr_pos in &layer_positions[layer_index] {
                                    painter.line_segment(
                                        [prev_pos, curr_pos],
                                        (2.0, egui::Color32::from_rgba_unmultiplied(200, 200, 200, 150)), // Główna linia
                                    );
                
                                    // Opcjonalnie: Gradient
                                    painter.line_segment(
                                        [prev_pos, curr_pos],
                                        (1.5, egui::Color32::WHITE), // Wewnętrzna jaśniejsza linia
                                    );
                                }
                            }
                        }
                    }
                });
                
                
            });
        });
    }
}


/// Funkcja przypisująca kolor warstwie
fn layer_color(layer_index: usize, total_layers: usize) -> egui::Color32 {
    if layer_index == 0 {
        egui::Color32::GREEN // Pierwsza warstwa
    } else if layer_index == total_layers - 1 {
        egui::Color32::RED // Ostatnia warstwa
    } else {
        egui::Color32::BLUE // Środkowe warstwy
    }
}
