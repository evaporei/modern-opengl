use gl::types::{GLboolean, GLchar, GLenum, GLint, GLuint};
use std::ffi::CStr;
use std::fs;

// Just Vertex and Fragment Shaders, no Geometric
const NUM_SHADERS: usize = 2;

pub struct Shader {
    program: GLuint,
    // TODO: maybe store two different variables, one for each shader
    shaders: [GLuint; NUM_SHADERS],
}

// TODO: make error handling better, like using Result
fn check_shader_error(shader: GLuint, flag: GLuint, is_program: bool, error_message: &str) {
    let mut success: GLint = 0;
    let mut error: [GLchar; 1024] = [0; 1024];

    if is_program {
        unsafe {
            gl::GetProgramiv(shader, flag, &mut success);
        }
    } else {
        unsafe {
            gl::GetShaderiv(shader, flag, &mut success);
        }
    }

    if success as GLboolean == gl::FALSE {
        if is_program {
            unsafe {
                gl::GetProgramInfoLog(
                    shader,
                    error.len() as i32,
                    std::ptr::null_mut(),
                    &mut error[0] as *mut i8,
                );
            }
        } else {
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    error.len() as i32,
                    std::ptr::null_mut(),
                    &mut error[0] as *mut i8,
                );
            }
        }

        let error_rust_str = {
            let c_str = unsafe { CStr::from_ptr(&error[0]) };
            c_str.to_str().unwrap()
        };

        eprintln!("{}: '{}'", error_message, error_rust_str)
    }
}

fn create_shader(text: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };

    // TODO: make better error handling
    if shader == 0 {
        eprintln!("Error: Shader creation failed");
    }

    let shader_source_strings: [*const GLchar; 1] = [text.as_ptr() as *const GLchar];
    let shader_source_string_lengths: [GLint; 1] = [text.len() as GLint];

    unsafe {
        gl::ShaderSource(
            shader,
            shader_source_string_lengths.len() as i32,
            shader_source_strings.as_ptr(),
            shader_source_string_lengths.as_ptr(),
        );
        gl::CompileShader(shader);
    }

    check_shader_error(
        shader,
        gl::COMPILE_STATUS,
        false,
        "Error: Shader compilation failed",
    );

    shader
}

impl Shader {
    pub fn new(file_name: &str) -> Self {
        let program = unsafe { gl::CreateProgram() };
        let shaders = [
            // TODO: create a read file string function, maybe Shader::new should return result
            // and combine with check_shader_error
            create_shader(
                &fs::read_to_string(format!("{}.vs", file_name)).unwrap(),
                gl::VERTEX_SHADER,
            ),
            create_shader(
                &fs::read_to_string(format!("{}.fs", file_name)).unwrap(),
                gl::FRAGMENT_SHADER,
            ),
        ];

        for shader in shaders.iter() {
            unsafe {
                gl::AttachShader(program, *shader);
            }
        }

        unsafe {
            gl::BindAttribLocation(program, 0, "position".as_ptr() as *const i8);
        }

        unsafe {
            gl::LinkProgram(program);
        }
        check_shader_error(
            program,
            gl::LINK_STATUS,
            true,
            "Error: Program linking failed: ",
        );

        unsafe {
            gl::ValidateProgram(program);
        }
        check_shader_error(
            program,
            gl::VALIDATE_STATUS,
            true,
            "Error: Program validation failed: ",
        );

        Self { program, shaders }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        for shader in self.shaders.iter() {
            unsafe {
                gl::DetachShader(self.program, *shader);
                gl::DeleteShader(*shader);
            }
        }

        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}
