pub mod input;
pub mod layer_shell;
pub mod session_lock;
pub mod xdg_shell;

use std::os::raw::c_void;
use std::ptr::NonNull;

use input::keyboard::KeyboardEvent;
use input::pointer::MouseEvent;
use input::touch::TouchEvent;
use mctk_core::component;
use mctk_core::raw_handle::RawWaylandHandle;
use raw_window_handle::{
    DisplayHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
    WindowHandle,
};
use wayland_client::protocol::wl_display::WlDisplay;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::Proxy;

pub struct PhysicalPosition<P> {
    pub x: P,
    pub y: P,
}

#[derive(Default, Clone)]
pub struct WindowOptions {
    pub height: u32,
    pub width: u32,
    pub scale_factor: f32,
}

#[derive(Default, Clone)]
pub struct WindowInfo {
    pub id: String,
    pub title: String,
    pub namespace: String,
}

#[derive(Debug)]
pub enum WindowMessage<'a> {
    Configure {
        width: u32,
        height: u32,
        wayland_handle: RawWaylandHandle<'a>,
    },
    CompositorFrame,
    MainEventsCleared,
    RedrawRequested,
    RequestNextFrame,
    Resize {
        width: u32,
        height: u32,
    },
    Send {
        message: component::Message,
    },
    WindowEvent {
        event: WindowEvent,
    },
}
unsafe impl Send for WindowMessage<'_> {}
#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
    CloseRequested,
    Focused,
    Unfocused,
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Touch(TouchEvent),
}

pub fn new_raw_wayland_handle<'a>(
    wl_display: &WlDisplay,
    wl_surface: &WlSurface,
) -> RawWaylandHandle<'a> {
    let wayland_handle = {
        let dislay_ptr = wl_display.id().as_ptr();
        let display =
            std::ptr::NonNull::new(dislay_ptr as *mut _).expect("wl_display should never be null");
        let mut handle = WaylandDisplayHandle::new(display);
        let display_handle = RawDisplayHandle::Wayland(handle);

        let surface_ptr = wl_surface.id().as_ptr();
        let surface =
            std::ptr::NonNull::new(surface_ptr as *mut _).expect("wl_surface should never be null");
        let mut handle = WaylandWindowHandle::new(surface);
        let window_handle = RawWindowHandle::Wayland(handle);

        RawWaylandHandle(
            unsafe { DisplayHandle::borrow_raw(display_handle.into()) },
            unsafe { WindowHandle::borrow_raw(window_handle.into()) },
        )
    };
    wayland_handle
}

mod reexports {
    pub use smithay_client_toolkit::reexports::calloop::channel::Sender;
}
