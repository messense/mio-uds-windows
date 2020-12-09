//! Mio bindings for Unix domain sockets on Windows

#![doc(html_root_url = "https://docs.rs/mio-uds-windows/0.1.0")]
#![deny(missing_docs, missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

mod listener;
mod poll;
mod stdnet;
mod stream;
mod sys;

pub mod net {
    //! The Windows equivalent of std::os::unix::net
    pub use crate::stdnet::{
        AcceptAddrs, AcceptAddrsBuf, SocketAddr, UnixListener, UnixListenerExt,
        UnixStream, UnixStreamExt
    };
}

pub use listener::UnixListener;
pub use stream::UnixStream;

