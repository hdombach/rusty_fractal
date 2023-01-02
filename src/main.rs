use eframe::egui::{self, ImageButton};
use eframe::epaint::{TextureHandle, Vec2};
use structures::scene::Scene;
use ui::scene_view::SceneView;
use ui::ui_traits::*;
use std::rc::Rc;
use std::sync::Arc;
use egui::mutex::Mutex;

pub mod util;
pub mod ui;
pub mod resources;
pub mod structures;
pub mod nodes;

fn main() {
    let options = eframe::NativeOptions {
        multisampling: 0,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 1,
        ..Default::default()
    };
    eframe::run_native(
        "Fractal Renderer",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    shared_state: Rc<ui::shared_state::SharedState>,
    main_scene: SceneView,
    left_panel_texture_id: usize,
    left_panel_texture_size: Vec2,
    right_panel_texture_id: usize,
    right_panel_texture_size: Vec2,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        let ui = &cc.egui_ctx;
        let scene = Arc::new(Mutex::new(Scene::default(gl).unwrap()));
        let shared_state = Rc::new(ui::shared_state::SharedState::new(ui));
        let left_panel_texture_id = shared_state
            .get_container()
            .get_texture_id("left_rectangle.png")
            .unwrap();
        let mut left_panel_texture_size = shared_state
            .get_container()
            .get_texture(left_panel_texture_id)
            .size_vec2();
        left_panel_texture_size = 16.0 * left_panel_texture_size / left_panel_texture_size.y;
        let right_panel_texture_id = shared_state
            .get_container()
            .get_texture_id("right_rectangle.png")
            .unwrap();
        let mut right_panel_texture_size = shared_state
            .get_container()
            .get_texture(right_panel_texture_id)
            .size_vec2();
        right_panel_texture_size = 16.0 * right_panel_texture_size / right_panel_texture_size.y;
        Self {
            shared_state: shared_state.clone(),
            main_scene: SceneView::new(scene, shared_state.clone()),
            left_panel_texture_id,
            left_panel_texture_size,
            right_panel_texture_id,
            right_panel_texture_size,
        }
    }

    fn get_left_panel_texture(&self) -> &TextureHandle {
        self.shared_state.get_container().get_texture(self.left_panel_texture_id)
    }
    fn get_right_panel_texture(&self) -> &TextureHandle {
        self.shared_state.get_container().get_texture(self.right_panel_texture_id)
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            let left_state = match self.main_scene.left_panel_state() {
                Some(value) => *value,
                None => false
            };
            let left_tint = match self.main_scene.has_left_panel() {
                true => match left_state {
                    true => ui.style().visuals.strong_text_color(),
                    false => ui.style().visuals.text_color(),
                }
                false => ui.style().visuals.weak_text_color(),
            };
            let right_state = match self.main_scene.right_panel_state() {
                Some(value) => *value,
                None => false,
            };
            let right_tint = match self.main_scene.has_right_panel() {
                true => match right_state {
                    true => ui.style().visuals.strong_text_color(),
                    false => ui.style().visuals.text_color(),
                }
                false => ui.style().visuals.weak_text_color(),
            };
            ui.horizontal(|ui| {
                if ui.add(ImageButton::new(self.get_left_panel_texture(), self.left_panel_texture_size).tint(left_tint)).clicked() {
                    if let Some(state) = self.main_scene.left_panel_state() {
                        *state = !state.clone();
                    }
                }
                if ui.add(ImageButton::new(self.get_right_panel_texture(), self.right_panel_texture_size).tint(right_tint)).clicked() {
                    if let Some(state) = self.main_scene.right_panel_state() {
                        *state = !state.clone();
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_scene.render(ui);
        });
        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {
        //TODO: tear down the scene.
    }
}
