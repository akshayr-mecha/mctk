use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WindowHandle,
};

#[derive(Debug, Clone, Copy)]
pub struct RawWaylandHandle<'a>(pub DisplayHandle<'a>, pub WindowHandle<'a>);

impl<'a> HasDisplayHandle for RawWaylandHandle<'a> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(self.0)
    }
}

impl<'a> HasWindowHandle for RawWaylandHandle<'a> {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        Ok(self.1)
    }
}

// This is safe because for wayland we can pass handles between threads
// ref: https://github.com/rust-windowing/raw-window-handle/issues/85
unsafe impl<'a> Send for RawWaylandHandle<'a> {}
unsafe impl<'a> Sync for RawWaylandHandle<'a> {}
