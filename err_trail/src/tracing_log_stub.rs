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
    fn trail_error(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as an "warn".
    fn trail_warn(self, context: impl Display) -> Result<T, E>;

    /// If [Err], lazily logging the result of [f] as an "error".
    fn trail_error_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as an "warn".
    fn trail_warn_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
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
    fn trail_error(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "warn".
    fn trail_warn(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn trail_error_end_with<F: FnOnce() -> D, D: Display>(self, f: F);
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "warn".
    fn trail_warn_end_with<F: FnOnce() -> D, D: Display>(self, f: F);
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
    fn trail_error_end(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "warn".
    /// Represents a bad state in which the current process can continue.
    fn trail_warn_end(self) -> Option<T>;

    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn trail_error_end_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "warn".
    /// Represents a bad state in which the current process can continue.
    fn trail_warn_end_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

//************************************************************************//

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ResultTrail<T, E> for Result<T, E> {
    #[inline]
    fn trail_error(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn trail_warn(self, context: impl Display) -> Result<T, E> {
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
    fn trail_error_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f(&err));
            #[cfg(feature = "log")]
            log::error!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn trail_warn_with<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
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
    fn trail_error_end(self) -> Option<T> {
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
    fn trail_warn_end(self) -> Option<T> {
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

    //************************************************************************//

    #[inline]
    fn trail_error_end_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", f());
                #[cfg(feature = "log")]
                log::error!("{}", f());
                None
            }
        }
    }

    fn trail_warn_end_with<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", f());
                #[cfg(feature = "log")]
                log::warn!("{}", f());
                None
            }
        }
    }
}

//************************************************************************//

impl<T> sealed::Sealed for Option<T> {}
impl<T> OptionTrail<T> for Option<T> {
    #[inline]
    fn trail_error(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
        }
        self
    }

    #[inline]
    fn trail_warn(self, context: impl Display) -> Option<T> {
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
    fn trail_error_end_with<F: FnOnce() -> D, D: Display>(self, f: F) {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
    }

    #[inline]
    fn trail_warn_end_with<F: FnOnce() -> D, D: Display>(self, f: F) {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f());
            #[cfg(feature = "log")]
            log::warn!("{}", f());
        }
    }
}
