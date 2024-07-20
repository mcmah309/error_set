use std::fmt::Display;

pub trait RecordContext<T, E> {
    fn error(self, context: impl Display) -> Result<T, E>;
    fn warn(self, context: impl Display) -> Result<T, E>;
    fn info(self, context: impl Display) -> Result<T, E>;
    fn debug(self, context: impl Display) -> Result<T, E>;
    fn trace(self, context: impl Display) -> Result<T, E>;
}

impl<T, E> RecordContext<T, E> for Result<T, E>
{
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
}
