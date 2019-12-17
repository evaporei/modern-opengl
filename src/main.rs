use modern_opengl::display::Display;

fn main() {
    let mut display = Display::new(800, 600, "Hello World!");

    while !display.is_closed {
        display.clear();
        display.update();
    }
}
