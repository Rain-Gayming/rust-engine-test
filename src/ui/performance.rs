use bevy::prelude::*;

use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_framepace::Limiter;

use crate::game::settings::GameSettings;

pub fn ui_example_system(mut contexts: EguiContexts, mut settings: ResMut<GameSettings>) {
    let _ = settings;
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("frame rate limit (Def: 60)");
        ui.add(egui::Slider::new(&mut settings.fps_limit, 20..=500));

        ui.label("render distance (Def:8)");
        ui.add(egui::Slider::new(&mut settings.render_distance, 2..=64));

        ui.label("vertical render distance (Def: 2)");
        ui.add(egui::Slider::new(
            &mut settings.vertical_render_distance,
            2..=64,
        ));
    });
}
