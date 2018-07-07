//! The backend interface and utilities.
//!
//! This crate contains the `Backend` trait, the `Event` type, the `Capability` enum, and all other
//! types and utilities that are needed to implement a Notify backend.
//!
//! # Examples
//!
//! Implementors should start by including the prelude:
//!
//! ```rust,ignore
//! extern crate notify_backend as backend;
//!
//! use backend::prelude::*;
//! ```
//!
//! And optionally the Buffer:
//!
//! ```rust,ignore
//! use backend::Buffer;
//! ```
//!
//! The prelude imports all types needed to implement a Backend, and re-exports dependent libraries
//! so there is no need to independently include them. Refer to the [implementor's guide] for a
//! thorough walk-through of backend implementation.

#![deny(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]

pub extern crate chrono;
pub extern crate futures;
pub extern crate mio;

pub use self::buffer::Buffer;

pub mod backend;
pub mod buffer;
pub mod capability;
pub mod event;
pub mod stream;

#[cfg(unix)]
pub mod unix;
#[macro_use]
pub mod compliance;

/// The Notify prelude.
///
/// All that is needed to implement a `Backend`, except for the optional `Buffer`.
pub mod prelude {
    pub use chrono;

    pub use futures::{self, Future, Poll, Stream};

    pub use mio::{
        self, event::Evented, Poll as MioPoll, PollOpt as MioPollOpt, Ready as MioReady,
        Registration as MioRegistration, Token as MioToken,
    };

    pub use std::path::PathBuf;

    /// An empty io::Result used for mio's Evented trait signatures
    pub type MioResult = ::std::io::Result<()>;

    pub use super::backend::{
        Backend as NotifyBackend, BoxedBackend, Error as BackendError,
        NewResult as NewBackendResult, ErrorWrap as BackendErrorWrap,
    };

    #[cfg(unix)]
    pub use super::unix::OwnedEventedFd;

    pub use super::capability::Capability;

    pub use super::event::{
        AccessKind, AccessMode, CreateKind, DataChange, DateTime, Event, EventKind, MetadataKind,
        ModifyKind, RemoveKind, RenameMode,
    };

    pub use super::stream::{
        EmptyResult as EmptyStreamResult, Error as StreamError, Item as StreamItem,
    };
}
