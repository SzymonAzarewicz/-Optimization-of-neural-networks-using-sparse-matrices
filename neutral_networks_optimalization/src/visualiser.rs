use eframe::egui;

pub fn run_visualiser(layers: Vec<usize>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
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
            let available_size = ui.available_size(); // Dostępna przestrzeń
            let total_layers = self.layers.len(); // Liczba warstw
            let max_neurons = *self.layers.iter().max().unwrap_or(&1); // Maksymalna liczba neuronów w jednej warstwie

            // Dynamiczne obliczanie rozmiaru neuronów
            let neuron_size = (available_size.y / (max_neurons as f32 + 2.0))
                .min(available_size.x / (total_layers as f32 + 3.0))
                .clamp(10.0, 50.0); // Rozmiar neuronów (10-50 px)

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
                            / 2.0;
                        ui.add_space(vertical_offset);

                        for _ in 0..neurons {
                            ui.add_space(vertical_spacing / 2.0);
                            let rect = ui.allocate_ui_with_layout(
                                egui::vec2(neuron_size, neuron_size),
                                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                |ui| {
                                    ui.painter().circle_filled(
                                        ui.max_rect().center(),
                                        neuron_size / 2.0,
                                        layer_color(layer_index, total_layers), // Kolor warstwy
                                    );
                                },
                            ).response.rect;

                            // Zapisujemy środek neuronu
                            current_layer_positions.push(rect.center());
                            ui.add_space(vertical_spacing / 2.0);
                        }
                    });

                    // Rysowanie połączeń z poprzednią warstwą
                    if let Some(prev_layer_positions) = layer_positions.last() {
                        let painter = ui.painter();
                        for &prev_pos in prev_layer_positions {
                            for &curr_pos in &current_layer_positions {
                                painter.line_segment(
                                    [prev_pos, curr_pos],
                                    (2.0, egui::Color32::GRAY), // Grubsze linie, jaśniejszy kolor
                                );
                            }
                        }
                    }

                    layer_positions.push(current_layer_positions);
                    ui.add_space(horizontal_spacing);
                }
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
