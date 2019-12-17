use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, GLProfile, Window};
use sdl2::{EventPump, Sdl};

pub struct Display {
    window: Window,
    event_pump: EventPump,
    pub is_closed: bool,
    // both below are just hold to keep context, since their Drop calls the Delete methods on C
    _gl_context: GLContext,
    _sdl_context: Sdl,
}

impl Display {
    pub fn new(width: usize, heigth: usize, title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        gl_attr.set_red_size(8);
        gl_attr.set_green_size(8);
        gl_attr.set_blue_size(8);
        gl_attr.set_buffer_size(32);
        gl_attr.set_double_buffer(true); // allocates space for atother window

        let window = video_subsystem
            .window(title, width as u32, heigth as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap(); // needs to be saved on struct because SDL_GL_DeleteContext is called when it is dropped
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        debug_assert_eq!(gl_attr.context_version(), (3, 3));
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);

        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            window,
            event_pump,
            is_closed: false,
            _gl_context,
            _sdl_context: sdl_context,
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.15, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn update(&mut self) {
        self.window.gl_swap_window();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_closed = true;
                }
                _ => {}
            }
        }
    }
}
