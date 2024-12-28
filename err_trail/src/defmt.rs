#![allow(unused_variables)]

use defmt::Format;

mod sealed {
    /// A sealed trait to prevent external implementations.
    pub trait Sealed {}
}

/// For logging a [Result] when [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
pub trait ErrContextDefmt<T, E>: sealed::Sealed {
    /// If [Err], log context as "error".
    fn error_context(self, context: impl Format) -> Result<T, E>;
    /// If [Err], log context as "warn".
    fn warn_context(self, context: impl Format) -> Result<T, E>;
    /// If [Err], log context as "info".
    fn info_context(self, context: impl Format) -> Result<T, E>;
    /// If [Err], log context as "debug".
    fn debug_context(self, context: impl Format) -> Result<T, E>;
    /// If [Err], log context as "trace".
    fn trace_context(self, context: impl Format) -> Result<T, E>;

    /// If [Err], lazily log result of [f] as "error".
    fn with_error_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily log result of [f] as "warn".
    fn with_warn_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily log result of [f] as "info".
    fn with_info_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily log result of [f] as "debug".
    fn with_debug_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// If [Err], lazily log result of [f] as "trace".
    fn with_trace_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;

    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "error".
    /// Represents a bad state in which the current process cannot continue.
    fn consume_with_error<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "warn".
    /// Represents a bad state in which the current process can continue.
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as an "info".
    /// Represents an information message.
    fn consume_with_info<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as a "debug".
    /// Represents a debug message.
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. If [Err], lazily logging the result of [f] as a "trace".
    /// Represents a trace message.
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T>;
}

/// For consuming a [Result]'s [Err] in [Format] when [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
pub trait ErrContextDisplayDefmt<T, E: Format>: sealed::Sealed {
    /// Consume [Err] of a [Result]. Log as "error".
    fn consume_as_error(self) -> Option<T>;
    /// Consume [Err] of a [Result]. Log as "warn".
    fn consume_as_warn(self) -> Option<T>;
    /// Consume [Err] of a [Result]. Log as "info".
    fn consume_as_info(self) -> Option<T>;
    /// Consume [Err] of a [Result]. Log as "debug".
    fn consume_as_debug(self) -> Option<T>;
    /// Consume [Err] of a [Result]. Log as "trace".
    fn consume_as_trace(self) -> Option<T>;
}

/// For logging an [Option] when [None] is encountered.
#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
pub trait NoneContextDefmt<T>: sealed::Sealed {
    /// If [None], log context as "error".
    fn error_context(self, context: impl Format) -> Option<T>;
    /// If [None], log context as "warn".
    fn warn_context(self, context: impl Format) -> Option<T>;
    /// If [None], log context as "info".
    fn info_context(self, context: impl Format) -> Option<T>;
    /// If [None], log context as "debug".
    fn debug_context(self, context: impl Format) -> Option<T>;
    /// If [None], log context as "trace".
    fn trace_context(self, context: impl Format) -> Option<T>;

    /// If [None], lazily log result of [f] as "error".
    fn with_error_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// If [None], lazily log result of [f] as "warn".
    fn with_warn_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// If [None], lazily log result of [f] as "info".
    fn with_info_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// If [None], lazily log result of [f] as "debug".
    fn with_debug_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// If [None], lazily log result of [f] as "trace".
    fn with_trace_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
}

//************************************************************************//

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ErrContextDefmt<T, E> for Result<T, E> {
    #[inline]
    fn error_context(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_context(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::warn!("{}", context);
        }
        self
    }

    #[inline]
    fn info_context(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug_context(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace_context(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::trace!("{}", context);
        }
        self
    }

    #[inline]
    fn with_error_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::error!("{}", f(err));
        }
        self
    }

    #[inline]
    fn with_warn_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::warn!("{}", f(err));
        }
        self
    }

    #[inline]
    fn with_info_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::info!("{}", f(err));
        }
        self
    }

    #[inline]
    fn with_debug_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::debug!("{}", f(err));
        }
        self
    }

    #[inline]
    fn with_trace_context<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::trace!("{}", f(err));
        }
        self
    }

    #[inline]
    fn consume_with_error<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                defmt::error!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                defmt::warn!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_info<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                defmt::info!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                defmt::debug!("{}", f(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                defmt::trace!("{}", f(&err));
                None
            }
        }
    }
}

impl<T, E: Format> ErrContextDisplayDefmt<T, E> for Result<T, E> {
    #[inline]
    fn consume_as_error(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::error!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_warn(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::warn!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_info(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::info!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_debug(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::debug!("{}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_as_trace(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::trace!("{}", err);
                None
            }
        }
    }
}

//************************************************************************//

impl<T> sealed::Sealed for Option<T> {}
impl<T> NoneContextDefmt<T> for Option<T> {
    #[inline]
    fn error_context(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn_context(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::warn!("{}", context);
        }
        self
    }

    #[inline]
    fn info_context(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug_context(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace_context(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::trace!("{}", context);
        }
        self
    }

    #[inline]
    fn with_error_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::error!("{}", f());
        }
        self
    }

    #[inline]
    fn with_warn_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::warn!("{}", f());
        }
        self
    }

    #[inline]
    fn with_info_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::info!("{}", f());
        }
        self
    }

    #[inline]
    fn with_debug_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::debug!("{}", f());
        }
        self
    }

    #[inline]
    fn with_trace_context<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::trace!("{}", f());
        }
        self
    }
} 
