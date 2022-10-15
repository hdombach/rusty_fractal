use eframe::egui;
use egui::mutex::Mutex;
use std::sync::Arc;

pub mod util;
pub mod ui;
pub mod resources;
pub mod structures;

use structures::scene::Scene;


fn main() {
    let options = eframe::NativeOptions {
        multisampling: 8,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    scene_state: ui::scene::SceneState,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            scene_state: ui::scene::SceneState::new(gl),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::scene::scene(ui, &mut self.scene_state);
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.scene_state.destroy(gl);
        }
    }
}
