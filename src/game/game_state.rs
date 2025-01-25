use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub is_paused: bool,
}
