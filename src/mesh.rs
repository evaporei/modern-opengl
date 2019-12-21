use gl::types::{GLint, GLuint};
use glm::Vec3;

pub struct Vertex {
    pos: Vec3,
}

impl Vertex {
    pub fn new(pos: Vec3) -> Self {
        Self { pos }
    }
}

const NUM_BUFFERS: usize = 1; // number of variants on enum below, it should be a macro
enum VertexArrayBuffer {
    PositionVB = 0,
}

pub struct Mesh {
    vertex_array_object: GLuint,
    vertex_array_buffers: [GLuint; NUM_BUFFERS],
    draw_count: usize,
}

impl Mesh {
    pub fn new(vertices: &[Vertex]) -> Self {
        let mut vertex_array_object_ptr: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_object_ptr);
            gl::BindVertexArray(vertex_array_object_ptr);
        }

        let mut vertex_array_buffers: [GLuint; NUM_BUFFERS] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        unsafe {
            gl::GenBuffers(NUM_BUFFERS as GLint, vertex_array_buffers.as_mut_ptr());
            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                vertex_array_buffers[VertexArrayBuffer::PositionVB as usize],
            );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0); // 0 -> all of the data is one attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

            gl::BindVertexArray(0);
        }

        Self {
            vertex_array_object: vertex_array_object_ptr,
            vertex_array_buffers,
            draw_count: vertices.len(),
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);

            gl::DrawArrays(gl::TRIANGLES, 0, self.draw_count as i32);

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array_object);
        }
    }
}
