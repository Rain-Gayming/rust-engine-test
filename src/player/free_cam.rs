use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::game::game_state::GameState;

pub fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    state: Res<GameState>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !state.is_paused {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            if keyboard_input.pressed(KeyCode::KeyW) {
                velocity += forward;
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                velocity -= forward;
            } else if keyboard_input.pressed(KeyCode::KeyA) {
                velocity -= right;
            } else if keyboard_input.pressed(KeyCode::KeyD) {
                velocity += right;
            } else if keyboard_input.pressed(KeyCode::Space) {
                velocity += Vec3::Y;
            } else if keyboard_input.pressed(KeyCode::ControlLeft) {
                velocity -= Vec3::Y;
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_secs() * 15.
        }
        let delta = accumulated_mouse_motion.delta;

        if delta != Vec2::ZERO {
            let delta_yaw = -delta.x * 0.003;
            let delta_pitch = -delta.y * 0.003;

            for mut transform in &mut query {
                let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
                let yaw = yaw + delta_yaw;

                const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
                let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

                transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
            }
        }
    }
}
