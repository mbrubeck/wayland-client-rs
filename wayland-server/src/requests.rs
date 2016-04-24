use std::iter::Iterator;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// All possible wayland requests.
///
/// This enum does a first sorting of request, with a variant for each
/// protocol extension activated in cargo features.
///
/// As the number of variant of this enum can change depending on which cargo features are
/// activated, it should *never* be matched exhaustively. It contains an hidden, never-used
/// variant to ensure it.
#[derive(Debug)]
pub enum Request {
    Wayland(::wayland::WaylandProtocolRequest),
    #[cfg(all(feature = "unstable-protocols", feature="wpu-xdg_shell"))]
    XdgShellUnstableV5(::xdg_shell::XdgShellUnstableV5ProtocolRequest),
    #[doc(hidden)]
    __DoNotMatchThis,
}

pub type RequestFifo = ::crossbeam::sync::MsQueue<Request>;

pub struct RequestIterator {
    fifo: Arc<(RequestFifo, AtomicBool)>
}

impl RequestIterator {
    pub fn new() -> RequestIterator {
        RequestIterator {
            fifo: Arc::new((::crossbeam::sync::MsQueue::new(),AtomicBool::new(true)))
        }
    }
}

impl Drop for RequestIterator {
    fn drop(&mut self) {
        self.fifo.1.store(false, Ordering::SeqCst);
    }
}

impl Iterator for RequestIterator {
    type Item = Request;
    fn next(&mut self) -> Option<Request> {
        self.fifo.0.pop()
    }
}

pub fn get_requestiter_internals(evt: &RequestIterator) -> Arc<(RequestFifo, AtomicBool)> {
    evt.fifo.clone()
}