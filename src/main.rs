use modern_opengl::display::Display;
use modern_opengl::shader::Shader;

fn main() {
    let mut display = Display::new(800, 600, "Hello World!");
    let shader = Shader::new("./res/shaders/basic_shader");

    while !display.is_closed {
        display.clear();

        shader.bind();

        display.update();
    }
}
