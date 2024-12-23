#![allow(unused_variables)]

use core::fmt::Display;

mod sealed {
    pub trait Sealed {}
}

/// For logging a [Result] when [Err] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
        feature = "context_stub"
    )))
)]
pub trait ResultTrail<T, E>: sealed::Sealed {
    /// If [Err], logging context as an "error".
    fn error_trail(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as an "warn".
    fn warn_trail(self, context: impl Display) -> Result<T, E>;

    /// If [Err], lazily logging the result of [f] as an "error".
    fn lazy_error_trail<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as an "warn".
    fn lazy_warn_trail<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
        feature = "context_stub"
    )))
)]
pub trait OptionTrail<T>: sealed::Sealed {
    /// If [None], logging context as an "error".
    fn error_trail(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "warn".
    fn warn_trail(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn lazy_error_trail<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "warn".
    fn lazy_warn_trail<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

/// For logging a [Result]'s [Err] in the [Display] format when an [Err] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
        feature = "context_stub"
    )))
)]
pub trait ResultTrailDisplay<T, E: Display>: sealed::Sealed {
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn error_trail_end(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "warn".
    /// Represents a bad state in which the current process can continue.
    fn warn_trail_end(self) -> Option<T>;
}

//************************************************************************//

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ResultTrail<T, E> for Result<T, E> {
    #[inline]
    fn error_trail(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_trail(self, context: impl Display) -> Result<T, E> {
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
    fn lazy_error_trail<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f(&err));
            #[cfg(feature = "log")]
            log::error!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn lazy_warn_trail<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
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

impl<T, E: Display> ResultTrailDisplay<T, E> for Result<T, E> {
    #[inline]
    fn error_trail_end(self) -> Option<T> {
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
    fn warn_trail_end(self) -> Option<T> {
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
impl<T> OptionTrail<T> for Option<T> {
    #[inline]
    fn error_trail(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_trail(self, context: impl Display) -> Option<T> {
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
    fn lazy_error_trail<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
        self
    }

    #[inline]
    fn lazy_warn_trail<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f());
            #[cfg(feature = "log")]
            log::warn!("{}", f());
        }
        self
    }
}
