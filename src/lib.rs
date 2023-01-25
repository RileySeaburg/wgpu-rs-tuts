use winit::{
event::{Event, WindowEvent},
event_loop::{ControlFlow, EventLoop},
window::Window,
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


/// # Name - run()
/// # Description - This function creates a window and event loop
/// # Arguments - None
/// # Return - None
/// # Example - run();
/// #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
/// tell wasm-bindgen to run our run() function when the WASM is loaded:
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
   let event_loop = EventLoop::new();
   let window = Window::new(&event_loop).unwrap();
   window.set_title("WGPU Winit");
   cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
    } else {
        env_logger::init();
    }
}


   event_loop.run(move | event, _, control_flow| {
       *control_flow = ControlFlow::Wait;
       match event {
           Event::WindowEvent {
               event: WindowEvent::CloseRequested,
       ..
     } => *control_flow = ControlFlow::Exit,
           _ => (),
       }
   });
}