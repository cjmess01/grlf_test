extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;
use std::sync::mpsc::Receiver;

fn main() {
    println!("Hello, World!");

    // This block initiates glfw, our windowing library.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // This block opens a window in glfw, with error handling.
    let (mut window, events) = glfw
        .create_window(800, 600, "Learn Open GL", glfw::WindowMode::Windowed)
        .expect("Expected window but failed");
    // Sets context to this window
    window.make_current();
    // Allows us to use key inputs
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Maps OpenGL pointers to real symbols.
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Central event loop.
    while !window.should_close() {
        // Processes any events that have occured
        process_events(&mut window, &events);

        // Sets color to clear to, and then clears
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    println!("Fin.");
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                window.maximize();
            }
            glfw::WindowEvent::Key(Key::B, _, Action::Press, _) => {
                window.set_title("New title");
            }
            _ => {}
        }
    }
}
