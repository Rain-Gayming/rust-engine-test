use raylib::prelude::*;

fn main() {
    //info on the window
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("voxel engine fr this time")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //renders a hello world text
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
