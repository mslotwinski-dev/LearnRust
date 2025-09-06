mod entity;
mod world;

use crate::world::World;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct App {
    deers: Arc<Mutex<Vec<usize>>>,
    boars: Arc<Mutex<Vec<usize>>>,
    wolves: Arc<Mutex<Vec<usize>>>,
    ravens: Arc<Mutex<Vec<usize>>>,
    corpses: Arc<Mutex<Vec<usize>>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plots = [
                ("Deers", &self.deers, egui::Color32::RED),
                ("Boars", &self.boars, egui::Color32::GREEN),
                ("Wolves", &self.wolves, egui::Color32::BLUE),
                ("Ravens", &self.ravens, egui::Color32::YELLOW),
                ("Corpses", &self.corpses, egui::Color32::LIGHT_GRAY),
            ];

            let columns = 3; // 3 kolumny
            let mut col_index = 0;

            ui.horizontal_wrapped(|ui| {
                for (label, data, color) in plots {
                    let data = data.lock().unwrap();
                    let points: Vec<[f64; 2]> = data
                        .iter()
                        .enumerate()
                        .map(|(x, y)| [x as f64, *y as f64])
                        .collect();

                    ui.vertical(|ui| {
                        ui.label(label);
                        Plot::new(label)
                            .view_aspect(1.0)
                            .width(550.0)
                            .height(400.0)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(label.to_string(), PlotPoints::from(points))
                                        .color(color),
                                );
                            });
                    });

                    col_index += 1;
                    if col_index % columns == 0 {
                        ui.end_row(); // przejście do nowego wiersza
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() {
    let world = Arc::new(Mutex::new(World::new()));

    let deers = Arc::new(Mutex::new(Vec::new()));
    let boars = Arc::new(Mutex::new(Vec::new()));
    let wolves = Arc::new(Mutex::new(Vec::new()));
    let ravens = Arc::new(Mutex::new(Vec::new()));
    let corpses = Arc::new(Mutex::new(Vec::new()));

    let deers_clone = Arc::clone(&deers);
    let boars_clone = Arc::clone(&boars);
    let wolves_clone = Arc::clone(&wolves);
    let ravens_clone = Arc::clone(&ravens);
    let corpses_clone = Arc::clone(&corpses);
    let world_clone = Arc::clone(&world);

    thread::spawn(move || {
        loop {
            let mut w = world_clone.lock().unwrap();
            w.simulation_step();
            let (d, b, wlf, r, c) = w.get_counts();

            deers_clone.lock().unwrap().push(d);
            boars_clone.lock().unwrap().push(b);
            wolves_clone.lock().unwrap().push(wlf);
            ravens_clone.lock().unwrap().push(r);
            corpses_clone.lock().unwrap().push(c);

            thread::sleep(Duration::from_millis(100));
        }
    });

    let app = App {
        deers,
        boars,
        wolves,
        ravens,
        corpses,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Symulacja ekosystemu",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}
