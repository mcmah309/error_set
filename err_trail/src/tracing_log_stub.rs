#![allow(unused_variables)]

use core::fmt::Display;

mod sealed {
    pub trait Sealed {}
}

/// For logging a [Result] when [Err] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "stub")))
)]
pub trait ErrContext<T, E>: sealed::Sealed {
    /// If [Err], logging context as an "error".
    fn error_context(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as an "warn".
    fn warn_context(self, context: impl Display) -> Result<T, E>;

    /// If [Err], lazily logging the result of [f] as an "error".
    fn with_error_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as an "warn".
    fn with_warn_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "stub")))
)]
pub trait NoneContext<T>: sealed::Sealed {
    /// If [None], logging context as an "error".
    fn error_context(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "warn".
    fn warn_context(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn with_error_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "warn".
    fn with_warn_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

/// For logging a [Result]'s [Err] in the [Display] format when an [Err] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "stub")))
)]
pub trait ErrContextDisplay<T, E: Display>: sealed::Sealed {
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn consume_as_error(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "warn".
    /// Represents a bad state in which the current process can continue.
    fn consume_as_warn(self) -> Option<T>;
}

//************************************************************************//

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ErrContext<T, E> for Result<T, E> {
    #[inline]
    fn error_context(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_context(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f(&err));
            #[cfg(feature = "log")]
            log::error!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_warn_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f(&err));
            #[cfg(feature = "log")]
            log::warn!("{}", f(&err));
        }
        self
    }
}

//************************************************************************//

impl<T, E: Display> ErrContextDisplay<T, E> for Result<T, E> {
    #[inline]
    fn consume_as_error(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", err);
                #[cfg(feature = "log")]
                log::error!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_warn(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", err);
                #[cfg(feature = "log")]
                log::warn!("{}", err);
                None
            }
        }
    }
}

//************************************************************************//

impl<T> sealed::Sealed for Option<T> {}
impl<T> NoneContext<T> for Option<T> {
    #[inline]
    fn error_context(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_context(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
        self
    }

    #[inline]
    fn with_warn_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f());
            #[cfg(feature = "log")]
            log::warn!("{}", f());
        }
        self
    }
}
