extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem.window("Window", 800, 600)
    .resizable()
    .opengl()
    .build()
    .unwrap();

    println!("{}", window.window_flags());

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 6));

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.08, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                
                Event::MouseMotion { timestamp: _, window_id : _, which : _, mousestate: _, x, y, xrel , yrel} => {
                    print!("x : {} y : {}", x, y);
                    print!("x rel: {} y rel : {}", xrel, yrel);
                },
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}