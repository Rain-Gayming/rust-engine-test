use raylib::prelude::*;

fn main() {
    //info on the window
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("voxel engine fr this time")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //renders the version text
        d.clear_background(Color::GRAY);
        d.draw_text("Voxel Engine v0.0.1a", 12, 12, 12, Color::BLACK);
    }
}
