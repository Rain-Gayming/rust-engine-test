use bevy::prelude::*;

use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::game::settings::GameSettings;

pub fn ui_example_system(mut contexts: EguiContexts, mut settings: ResMut<GameSettings>) {
    let _ = settings;
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        ui.add(egui::Slider::new(&mut settings.fps_limit, 20..=500));
        ui.add(egui::Slider::new(&mut settings.render_distance, 2..=128));
        ui.add(egui::Slider::new(
            &mut settings.vertical_render_distance,
            2..=128,
        ));
    });
}
