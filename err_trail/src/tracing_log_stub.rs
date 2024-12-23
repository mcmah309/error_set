#![allow(unused_variables)]

use core::fmt::Display;

mod sealed {
    pub trait Sealed {}
}

/// For logging a [Result] when [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log", feature = "defmt", feature = "context_stub"))))]
pub trait ResultTrail<T, E>: sealed::Sealed {
    /// Consumes the [Err] of a Result. If [Err], logging context as an "error".
    fn end_trail(self, context: impl Display) -> Option<T>;
    /// If [Err], logging context as an "error".
    fn add_trail(self, context: impl Display) -> Result<T, E>;

    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "error".
    fn end_trail_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// If [Err], lazily logging the result of [f] as an "error".
    fn add_trail_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log", feature = "defmt", feature = "context_stub"))))]
pub trait OptionTrail<T>: sealed::Sealed {
    /// Consumes the [Option]. If [None], logging context as an "error".
    fn end_trail(self, context: impl Display);
    /// If [None], logging context as an "error".
    fn add_trail(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn end_trail_with<F: FnOnce() -> D, D: Display>(self, f: F);
    /// If [None], lazily logging the result of [f] as an "error".
    fn add_trail_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

//************************************************************************//

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ResultTrail<T, E> for Result<T, E> {
    #[inline]
    fn end_trail(self, context: impl Display) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(_) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", context);
                #[cfg(feature = "log")]
                log::error!("{}", context);
                None   
            },
        }
    }

    #[inline]
    fn add_trail(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn end_trail_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", f(&err));
                #[cfg(feature = "log")]
                log::error!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn add_trail_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f(&err));
            #[cfg(feature = "log")]
            log::error!("{}", f(&err));
        }
        self
    }
}

//************************************************************************//

impl<T> sealed::Sealed for Option<T> {}
impl<T> OptionTrail<T> for Option<T> {
    #[inline]
    fn end_trail(self, context: impl Display) {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
    }

    #[inline]
    fn add_trail(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn end_trail_with<F: FnOnce() -> D, D: Display>(self, f: F) {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
    }

    #[inline]
    fn add_trail_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
        self
    }
}
