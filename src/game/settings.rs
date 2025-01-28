use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameSettings {
    //graphicals
    pub fps_limit: i32,
    pub render_distance: i32,
    pub vertical_render_distance: i32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            fps_limit: 60,
            render_distance: 8,
            vertical_render_distance: 2,
        }
    }
}
