#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use core::fmt::Display;

mod sealed {
    /// A sealed trait to prevent external implementations.
    pub trait Sealed {}
}

/// For logging a [`Result`] when [`Result::Err`] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
    )))
)]
pub trait ErrContext<T, E>: sealed::Sealed {
    /// If [`Result::Err`], logging context as an "error".
    fn error(self, context: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "warn".
    fn warn(self, context: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as an "info".
    fn info(self, context: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "debug".
    fn debug(self, context: impl Display) -> Result<T, E>;
    /// If [`Result::Err`], logging context as a "trace".
    fn trace(self, context: impl Display) -> Result<T, E>;

    /// If [`Result::Err`], lazily logging the result of [f] as an "error".
    fn with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "warn".
    fn with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as an "info".
    fn with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "debug".
    fn with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// If [`Result::Err`], lazily logging the result of [f] as a "trace".
    fn with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;

    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], lazily logging the result of [f] as an "error".
    // Represents a bad state in which the current process cannot continue.
    fn consume_with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], lazily logging the result of [f] as a "warn".
    // Represents a bad state in which the current process can continue.
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], lazily logging the result of [f] as an "info".
    fn consume_with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], lazily logging the result of [f] as a "debug".
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], lazily logging the result of [f] as a "trace".
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T>;
}

/// For logging a [`Result`] when [`Result::Err`] is encountered and [E] is [`Display`]
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
    )))
)]
pub trait ErrContextDisplay<T, E: Display>: ErrContext<T, E> + sealed::Sealed {
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], logging the display of the error as an "error".
    // Represents a bad state in which the current process cannot continue.
    fn consume_error(self) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], logging the display of the error as a "warn".
    // Represents a bad state in which the current process can continue.
    fn consume_warn(self) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], logging the display of the error as an "info".
    fn consume_info(self) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], logging the display of the error as a "debug".
    fn consume_debug(self) -> Option<T>;
    /// Consumes the [`Result::Err`] of a Result. If [`Result::Err`], logging the display of the error as a "trace".
    fn consume_trace(self) -> Option<T>;
}

/// For logging a [Option] when [None] is encountered.
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "tracing",
        feature = "log",
        feature = "defmt",
    )))
)]
pub trait NoneContext<T>: sealed::Sealed {
    /// If [None], logging context as an "error".
    fn error(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "warn".
    fn warn(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as an "info".
    fn info(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "debug".
    fn debug(self, context: impl Display) -> Option<T>;
    /// If [None], logging context as a "trace".
    fn trace(self, context: impl Display) -> Option<T>;

    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "error".
    fn with_error<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "warn".
    fn with_warn<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as an "info".
    fn with_info<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "debug".
    fn with_debug<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Option]. If [None], lazily logging the result of [f] as a "trace".
    fn with_trace<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

impl<T, E> sealed::Sealed for Result<T, E> {}

impl<T, E> ErrContext<T, E> for Result<T, E> {
    #[inline]
    fn error(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn info(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f(&err);
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn consume_with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
                let context = f(&err);
                #[cfg(feature = "tracing")]
                tracing::error!("{}", context);
                #[cfg(feature = "log")]
                log::error!("{}", context);
                #[cfg(feature = "defmt")]
                defmt::error!("{}", defmt::Display2Format(&context));
                None
            }
        }
    }

    #[inline]
    fn consume_with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
                let context = f(&err);
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", context);
                #[cfg(feature = "log")]
                log::warn!("{}", context);
                #[cfg(feature = "defmt")]
                defmt::warn!("{}", defmt::Display2Format(&context));
                None
            }
        }
    }

    #[inline]
    fn consume_with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
                let context = f(&err);
                #[cfg(feature = "tracing")]
                tracing::info!("{}", context);
                #[cfg(feature = "log")]
                log::info!("{}", context);
                #[cfg(feature = "defmt")]
                defmt::info!("{}", defmt::Display2Format(&context));
                None
            }
        }
    }

    #[inline]
    fn consume_with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
                let context = f(&err);
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", context);
                #[cfg(feature = "log")]
                log::debug!("{}", context);
                #[cfg(feature = "defmt")]
                defmt::debug!("{}", defmt::Display2Format(&context));
                None
            }
        }
    }

    #[inline]
    fn consume_with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
                let context = f(&err);
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", context);
                #[cfg(feature = "log")]
                log::trace!("{}", context);
                #[cfg(feature = "defmt")]
                defmt::trace!("{}", defmt::Display2Format(&context));
                None
            }
        }
    }
}

impl<T, E: Display> ErrContextDisplay<T, E> for Result<T, E> {
    #[inline]
    fn consume_error(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", err);
                #[cfg(feature = "log")]
                log::error!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::error!("{}", defmt::Display2Format(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_warn(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", err);
                #[cfg(feature = "log")]
                log::warn!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::warn!("{}", defmt::Display2Format(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_info(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{}", err);
                #[cfg(feature = "log")]
                log::info!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::info!("{}", defmt::Display2Format(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_debug(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", err);
                #[cfg(feature = "log")]
                log::debug!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::debug!("{}", defmt::Display2Format(&err));
                None
            }
        }
    }

    #[inline]
    fn consume_trace(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", err);
                #[cfg(feature = "log")]
                log::trace!("{}", err);
                #[cfg(feature = "defmt")]
                defmt::trace!("{}", defmt::Display2Format(&err));
                None
            }
        }
    }
}

impl<T> sealed::Sealed for Option<T> {}

impl<T> NoneContext<T> for Option<T> {
    #[inline]
    fn error(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn info(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_error<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::error!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", context);
            #[cfg(feature = "log")]
            log::warn!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::warn!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::info!("{}", context);
            #[cfg(feature = "log")]
            log::info!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::info!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", context);
            #[cfg(feature = "log")]
            log::debug!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::debug!("{}", defmt::Display2Format(&context));
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
            let context = f();
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", context);
            #[cfg(feature = "log")]
            log::trace!("{}", context);
            #[cfg(feature = "defmt")]
            defmt::trace!("{}", defmt::Display2Format(&context));
        }
        self
    }
}
