#![allow(unused_variables)]

use core::fmt::Display;

mod sealed {
    /// A sealed trait to prevent external implementations.
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
    /// If [Err], logging context as a "warn".
    fn warn_context(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as an "info".
    fn info_context(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as a "debug".
    fn debug_context(self, context: impl Display) -> Result<T, E>;
    /// If [Err], logging context as a "trace".
    fn trace_context(self, context: impl Display) -> Result<T, E>;

    /// If [Err], lazily logging the result of [f] as an "error".
    fn with_error_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as a "warn".
    fn with_warn_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as an "info".
    fn with_info_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as a "debug".
    fn with_debug_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily logging the result of [f] as a "trace".
    fn with_trace_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;

    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn consume_with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as a "warn".
    /// Represents a bad state in which the current process can continue.
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "info".
    fn consume_with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as a "debug".
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as a "trace".
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
}

/// For logging a [Result] when [Err] is encountered and [E] is [Display]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "stub")))
)]
pub trait ErrContextDisplay<T, E: Display>: ErrContext<T, E> + sealed::Sealed {
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn consume_as_error(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as a "warn".
    /// Represents a bad state in which the current process can continue.
    fn consume_as_warn(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as an "info".
    fn consume_as_info(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as a "debug".
    fn consume_as_debug(self) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], logging the display of the error as a "trace".
    fn consume_as_trace(self) -> Option<T>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tracing", feature = "log", feature = "stub")))
)]
pub trait NoneContext<T>: sealed::Sealed {
    /// If [None], logging context as an "error".
    fn error_context(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "warn".
    fn warn_context(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "info".
    fn info_context(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "debug".
    fn debug_context(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "trace".
    fn trace_context(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn with_error_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "warn".
    fn with_warn_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "info".
    fn with_info_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "debug".
    fn with_debug_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "trace".
    fn with_trace_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

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

    #[inline]
    fn info_context(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug_context(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace_context(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
        }
        self
    }

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

    #[inline]
    fn with_info_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", f(&err));
            #[cfg(feature = "log")]
            log::info!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_debug_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", f(&err));
            #[cfg(feature = "log")]
            log::debug!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_trace_context<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", f(&err));
            #[cfg(feature = "log")]
            log::trace!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn consume_with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
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
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", f(&err));
                #[cfg(feature = "log")]
                log::warn!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{}", f(&err));
                #[cfg(feature = "log")]
                log::info!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", f(&err));
                #[cfg(feature = "log")]
                log::debug!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", f(&err));
                #[cfg(feature = "log")]
                log::trace!("{}", f(&err));
                None
            }
        }
    }
}

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

    #[inline]
    fn consume_as_info(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{}", err);
                #[cfg(feature = "log")]
                log::info!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_debug(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", err);
                #[cfg(feature = "log")]
                log::debug!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_trace(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", err);
                #[cfg(feature = "log")]
                log::trace!("{}", err);
                None
            }
        }
    }
}

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

    #[inline]
    fn info_context(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug_context(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace_context(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
        }
        self
    }

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

    #[inline]
    fn with_info_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", f());
            #[cfg(feature = "log")]
            log::info!("{}", f());
        }
        self
    }

    #[inline]
    fn with_debug_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", f());
            #[cfg(feature = "log")]
            log::debug!("{}", f());
        }
        self
    }

    #[inline]
    fn with_trace_context<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", f());
            #[cfg(feature = "log")]
            log::trace!("{}", f());
        }
        self
    }
}
