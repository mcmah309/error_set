#![allow(unused_variables)]

use core::fmt::Debug;
use core::fmt::Display;

mod sealed {
    pub trait Sealed {}
}

/// For logging a [Result] when an [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log"))))]
pub trait ResultContext<T, E>: sealed::Sealed {
    /// Log the context as an "error" if the Result is an [Err].
    fn error(self, context: impl Display) -> Result<T, E>;
    /// Log the context as an "warn" if the Result is an [Err].
    fn warn(self, context: impl Display) -> Result<T, E>;
    /// Log the context as an "info" if the Result is an [Err].
    fn info(self, context: impl Display) -> Result<T, E>;
    /// Log the context as an "debug" if the Result is an [Err].
    fn debug(self, context: impl Display) -> Result<T, E>;
    /// Log the context as an "trace" if the Result is an [Err].
    fn trace(self, context: impl Display) -> Result<T, E>;

    /// Lazily call [f] if the Result is an [Err] and log as an "error".
    fn with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "warn".
    fn with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "info".
    fn with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "debug".
    fn with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "trace".
    fn with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E>;

    /// Consumes the [Err] of a Result. if [Err], logging as an "error" with the result of [f].
    fn consume_with_error<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "warn" with the result of [f].
    fn consume_with_warn<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "info" with the result of [f].
    fn consume_with_info<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "debug" with the result of [f].
    fn consume_with_debug<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "trace" with the result of [f].
    fn consume_with_trace<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T>;
}

/// For logging a [Result]'s [Err] in the [Debug] format when an [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log"))))]
pub trait ConsumeDebug<T, E>: sealed::Sealed {
    /// Consumes the [Err] of a Result. if [Err], logging as an "error".
    fn consume_error(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "warn".
    fn consume_warn(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "info".
    fn consume_info(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "debug".
    fn consume_debug(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "trace".
    fn consume_trace(self) -> Option<T>;
}

/// For logging a [Result]'s [Err] in the [Display] format when an [Err] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log"))))]
pub trait ConsumeDisplay<T, E>: sealed::Sealed {
    /// Consumes the [Err] of a Result. if [Err], logging as an "error".
    fn consume_error(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "warn".
    fn consume_warn(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "info".
    fn consume_info(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "debug".
    fn consume_debug(self) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "trace".
    fn consume_trace(self) -> Option<T>;
}

/// For logging when a [None] is encountered.
#[cfg_attr(docsrs, doc(cfg(any(feature = "tracing", feature = "log"))))]
pub trait OptionContext<T>: sealed::Sealed {
    /// Log the context as an "error" if the Option is [None].
    fn error(self, context: impl Display) -> Option<T>;
    /// Log the context as an "warn" if the Option is [None].
    fn warn(self, context: impl Display) -> Option<T>;
    /// Log the context as an "info" if the Option is [None].
    fn info(self, context: impl Display) -> Option<T>;
    /// Log the context as an "debug" if the Option is [None].
    fn debug(self, context: impl Display) -> Option<T>;
    /// Log the context as an "trace" if the Option is [None].
    fn trace(self, context: impl Display) -> Option<T>;

    /// Lazily call [f] if the Option is [None] and log as an "error".
    fn with_error<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "warn".
    fn with_warn<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "info".
    fn with_info<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "debug".
    fn with_debug<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "trace".
    fn with_trace<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T>;
}

impl<T, E> sealed::Sealed for Result<T, E> {}
impl<T, E> ResultContext<T, E> for Result<T, E> {
    #[inline]
    fn error(self, context: impl Display) -> Result<T, E> {
        if self.is_err() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
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
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f(&err));
            #[cfg(feature = "log")]
            log::error!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f(&err));
            #[cfg(feature = "log")]
            log::warn!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", f(&err));
            #[cfg(feature = "log")]
            log::info!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", f(&err));
            #[cfg(feature = "log")]
            log::debug!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce(&E) -> D, D: Display>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", f(&err));
            #[cfg(feature = "log")]
            log::trace!("{}", f(&err));
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn consume_with_error<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{}", f(err));
                #[cfg(feature = "log")]
                log::error!("{}", f(err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_warn<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", f(err));
                #[cfg(feature = "log")]
                log::warn!("{}", f(err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_info<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{}", f(err));
                #[cfg(feature = "log")]
                log::info!("{}", f(err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_debug<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{}", f(err));
                #[cfg(feature = "log")]
                log::debug!("{}", f(err));
                None
            }
        }
    }

    #[inline]
    fn consume_with_trace<F: FnOnce(E) -> D, D: Display>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{}", f(err));
                #[cfg(feature = "log")]
                log::trace!("{}", f(err));
                None
            }
        }
    }
}

impl<T, E> ConsumeDebug<T, E> for Result<T, E>
where
    E: Debug,
{
    #[inline]
    fn consume_error(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!("{:?}", err);
                #[cfg(feature = "log")]
                log::error!("{:?}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_warn(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!("{:?}", err);
                #[cfg(feature = "log")]
                log::warn!("{:?}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_info(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{:?}", err);
                #[cfg(feature = "log")]
                log::info!("{:?}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_debug(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::debug!("{:?}", err);
                #[cfg(feature = "log")]
                log::debug!("{:?}", err);
                None
            }
        }
    }

    #[inline]
    fn consume_trace(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::trace!("{:?}", err);
                #[cfg(feature = "log")]
                log::trace!("{:?}", err);
                None
            }
        }
    }
}

impl<T, E> ConsumeDisplay<T, E> for Result<T, E>
where
    E: Display,
{
    #[inline]
    fn consume_error(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
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
    fn consume_warn(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
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
    fn consume_info(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
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
    fn consume_debug(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
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
    fn consume_trace(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
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

//************************************************************************//

impl<T> sealed::Sealed for Option<T> {}
impl<T> OptionContext<T> for Option<T> {
    #[inline]
    fn error(self, context: impl Display) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", context);
            #[cfg(feature = "log")]
            log::error!("{}", context);
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
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::error!("{}", f());
            #[cfg(feature = "log")]
            log::error!("{}", f());
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}", f());
            #[cfg(feature = "log")]
            log::warn!("{}", f());
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", f());
            #[cfg(feature = "log")]
            log::info!("{}", f());
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", f());
            #[cfg(feature = "log")]
            log::debug!("{}", f());
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce() -> D, D: Display>(self, f: F) -> Option<T> {
        if self.is_none() {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}", f());
            #[cfg(feature = "log")]
            log::trace!("{}", f());
        }
        self
    }
}
