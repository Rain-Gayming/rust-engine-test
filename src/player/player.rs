use bevy::{math::IVec3, prelude::Component};

#[derive(Component)]
pub struct Player {
    pub chunk_position: IVec3,
    pub in_new_chunk: bool,
}
