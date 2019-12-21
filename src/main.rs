use glm::vec3;
use modern_opengl::display::Display;
use modern_opengl::mesh::{Mesh, Vertex};
use modern_opengl::shader::Shader;

fn main() {
    let mut display = Display::new(800, 600, "Hello World!");
    // OpenGL cordinates:
    //
    //          1
    //
    // -1               1  --------------- x
    //
    //         -1
    //
    //          |
    //          |
    //          |
    //          |
    //
    //          y
    //
    let vertices = [
        Vertex::new(vec3(-0.5, -0.5, 0.0)),
        Vertex::new(vec3(0.0, 0.5, 0.0)),
        Vertex::new(vec3(0.5, -0.5, 0.0)),
    ];

    let mesh = Mesh::new(&vertices);

    let shader = Shader::new("./res/shaders/basic_shader");

    while !display.is_closed {
        display.clear();

        shader.bind();
        mesh.draw();

        display.update();
    }
}
