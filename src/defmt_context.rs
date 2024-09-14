#![allow(unused_variables)]

use defmt::Format;

/// For logging a [Result] when an [Err] is encountered.
pub trait ResultContext<T, E> {
    /// Log the context as an "error" if the Result is an [Err].
    fn error(self, context: impl Format) -> Result<T, E>;
    /// Log the context as an "warn" if the Result is an [Err].
    fn warn(self, context: impl Format) -> Result<T, E>;
    /// Log the context as an "info" if the Result is an [Err].
    fn info(self, context: impl Format) -> Result<T, E>;
    /// Log the context as an "debug" if the Result is an [Err].
    fn debug(self, context: impl Format) -> Result<T, E>;
    /// Log the context as an "trace" if the Result is an [Err].
    fn trace(self, context: impl Format) -> Result<T, E>;

    /// Lazily call [f] if the Result is an [Err] and log as an "error".
    fn with_error<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "warn".
    fn with_warn<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "info".
    fn with_info<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "debug".
    fn with_debug<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;
    /// Lazily call [f] if the Result is an [Err] and log as an "trace".
    fn with_trace<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E>;

    /// Consumes the [Err] of a Result. if [Err], logging as an "error" with the result of [f].
    fn consume_with_error<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "warn" with the result of [f].
    fn consume_with_warn<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "info" with the result of [f].
    fn consume_with_info<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "debug" with the result of [f].
    fn consume_with_debug<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T>;
    /// Consumes the [Err] of a Result. if [Err], logging as an "trace" with the result of [f].
    fn consume_with_trace<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T>;
}

/// For logging a [Result] when an [Err] is encountered and [Result] implements [Debug].
pub trait ResultContextDebug<T, E>: ResultContext<T, E> {
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
pub trait OptionContext<T> {
    /// Log the context as an "error" if the Option is [None].
    fn error(self, context: impl Format) -> Option<T>;
    /// Log the context as an "warn" if the Option is [None].
    fn warn(self, context: impl Format) -> Option<T>;
    /// Log the context as an "info" if the Option is [None].
    fn info(self, context: impl Format) -> Option<T>;
    /// Log the context as an "debug" if the Option is [None].
    fn debug(self, context: impl Format) -> Option<T>;
    /// Log the context as an "trace" if the Option is [None].
    fn trace(self, context: impl Format) -> Option<T>;

    /// Lazily call [f] if the Option is [None] and log as an "error".
    fn with_error<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "warn".
    fn with_warn<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "info".
    fn with_info<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "debug".
    fn with_debug<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
    /// Lazily call [f] if the Option is [None] and log as an "trace".
    fn with_trace<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T>;
}

impl<T, E> ResultContext<T, E> for Result<T, E> {
    #[inline]
    fn error(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::warn!("{}", context);
        }
        self
    }

    #[inline]
    fn info(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Format) -> Result<T, E> {
        if self.is_err() {
            defmt::trace!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::error!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::warn!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::info!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::debug!("{}", f(&err));
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce(&E) -> D, D: Format>(self, f: F) -> Result<T, E> {
        if let Err(err) = &self {
            defmt::trace!("{}", f(&err));
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn consume_with_error<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::error!("{}", f(err));

                None
            }
        }
    }

    #[inline]
    fn consume_with_warn<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::warn!("{}", f(err));

                None
            }
        }
    }

    #[inline]
    fn consume_with_info<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::info!("{}", f(err));

                None
            }
        }
    }

    #[inline]
    fn consume_with_debug<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::debug!("{}", f(err));

                None
            }
        }
    }

    #[inline]
    fn consume_with_trace<F: FnOnce(E) -> D, D: Format>(self, f: F) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::trace!("{}", f(err));

                None
            }
        }
    }
}

impl<T, E> ResultContextDebug<T, E> for Result<T, E>
where
    E: Format,
{
    #[inline]
    fn consume_error(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::error!("{:?}", err);

                None
            }
        }
    }

    #[inline]
    fn consume_warn(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::warn!("{:?}", err);

                None
            }
        }
    }

    #[inline]
    fn consume_info(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::info!("{:?}", err);

                None
            }
        }
    }

    #[inline]
    fn consume_debug(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::debug!("{:?}", err);

                None
            }
        }
    }

    #[inline]
    fn consume_trace(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                defmt::trace!("{:?}", err);

                None
            }
        }
    }
}

//************************************************************************//

impl<T> OptionContext<T> for Option<T> {
    #[inline]
    fn error(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::error!("{}", context);
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::warn!("{}", context);
        }
        self
    }

    #[inline]
    fn info(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::info!("{}", context);
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::debug!("{}", context);
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Format) -> Option<T> {
        if self.is_none() {
            defmt::trace!("{}", context);
        }
        self
    }

    //************************************************************************//

    #[inline]
    fn with_error<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::error!("{}", f());
        }
        self
    }

    #[inline]
    fn with_warn<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::warn!("{}", f());
        }
        self
    }

    #[inline]
    fn with_info<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::info!("{}", f());
        }
        self
    }

    #[inline]
    fn with_debug<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::debug!("{}", f());
        }
        self
    }

    #[inline]
    fn with_trace<F: FnOnce() -> D, D: Format>(self, f: F) -> Option<T> {
        if self.is_none() {
            defmt::trace!("{}", f());
        }
        self
    }
}
