use crate::SmallWmState;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::renderer::Frame;
use smithay::backend::renderer::Renderer;
use smithay::backend::winit::{self, WinitEvent};
use smithay::input::SeatState;
use smithay::reexports::ash::vk::Queue;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use smithay::utils::{Rectangle, Transform};
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::shm::ShmState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn run_window() {
    let mut event_loop: EventLoop<SmallWmState> = EventLoop::try_new().unwrap();
    let display: Display<SmallWmState> = Display::new().unwrap();
    let dh = display.handle();

    let compositor_state = CompositorState::new::<SmallWmState>(&dh);
    let shm_state = ShmState::new::<SmallWmState>(&dh, vec![]);
    let seat_state = SeatState::new();
    let xdg_shell_state = XdgShellState::new::<SmallWmState>(&dh);

    let mut state = SmallWmState {
        compositor_state,
        shm_state,
        seat_state,
        xdg_shell_state,
        toplevels: Vec::new(),
    };

    let (backend, winit_event_source) = winit::init::<GlesRenderer>().unwrap();
    let backend = Rc::new(RefCell::new(backend));

    backend.borrow_mut().window().request_redraw();

    let backend_clone = backend.clone();
    event_loop
        .handle()
        .insert_source(winit_event_source, move |event, _, _state| {
            let backend = backend_clone.borrow_mut();
            match event {
                WinitEvent::Resized { size, scale_factor, .. } => {
                    backend.window().request_redraw();
                    let _ = (size, scale_factor);
                }
                WinitEvent::Input(event) => {
                    let _ = event;
                }
                WinitEvent::CloseRequested => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .unwrap();

    println!("Smithay winit backend initialized. Starting event loop...");

    event_loop
        .run(std::time::Duration::from_millis(16), &mut state, |state| {
            let mut backend = backend.borrow_mut();
            let size = backend.window_size();
            let damage = Rectangle::from_size(size);

            let toplevels = state.toplevels.clone();

            if let Ok(()) = backend.bind() {
                let renderer = backend.renderer();

                for toplevel in &toplevels {
                    let wl_surface = toplevel.wl_surface();
                    // Pass renderer directly without an extra &mut if backend.renderer() 
                    // already returns a mutable renderer handle, or pass &mut *renderer.
                    let _ = smithay::backend::renderer::utils::import_surface_tree(
                        &mut *renderer,
                        wl_surface,
                    );

                }

                if let Ok(mut frame) = renderer.render(size, Transform::Normal) {
                    let _ = frame.clear([0.1, 0.1, 0.1, 1.0].into(), &[damage]);
                }
                let _ = backend.submit(Some(&[damage]));
            }
        })
        .unwrap();
}
