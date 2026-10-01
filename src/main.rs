mod window;

use smithay::delegate_compositor;
use smithay::delegate_seat;
use smithay::delegate_shm;
use smithay::delegate_xdg_shell;
use smithay::input::{SeatHandler, SeatState};
use smithay::reexports::wayland_server::backend::ClientData;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Client;
use smithay::wayland::buffer::BufferHandler;
use smithay::wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState};
use smithay::wayland::shell::xdg::{
    PositionerState, PopupSurface, ToplevelSurface, XdgShellHandler, XdgShellState,
};
use smithay::wayland::shm::{ShmHandler, ShmState};

#[derive(Default)]
pub struct CustomClientData {
    pub compositor_state: CompositorClientState,
}

impl ClientData for CustomClientData {}

pub struct SmallWmState {
    pub compositor_state: CompositorState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<SmallWmState>,
    pub xdg_shell_state: XdgShellState,
    pub toplevels: Vec<ToplevelSurface>,
}

impl CompositorHandler for SmallWmState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<CustomClientData>()
            .unwrap()
            .compositor_state
    }

    fn commit(&mut self, _surface: &WlSurface) {}
}

impl BufferHandler for SmallWmState {
    fn buffer_destroyed(&mut self, _buffer: &WlBuffer) {}
}

impl ShmHandler for SmallWmState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for SmallWmState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &smithay::input::Seat<Self>, _focus: Option<&WlSurface>) {}
    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

impl XdgShellHandler for SmallWmState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.send_configure();
        self.toplevels.push(surface);
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}
    fn grab(&mut self, _surface: smithay::wayland::shell::xdg::PopupSurface, _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat, _serial: smithay::utils::Serial) {}
    fn reposition_request(&mut self, _surface: PopupSurface, _positioner: PositionerState, _token: u32) {}
}

delegate_seat!(SmallWmState);
delegate_compositor!(SmallWmState);
delegate_shm!(SmallWmState);
delegate_xdg_shell!(SmallWmState);

fn main() {
    window::run_window();
}
