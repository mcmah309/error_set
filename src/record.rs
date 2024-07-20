use std::fmt::Display;

pub trait RecordErr<T, E> {
    fn error(self, context: impl Display) -> Result<T, E>;
    fn warn(self, context: impl Display) -> Result<T, E>;
    fn info(self, context: impl Display) -> Result<T, E>;
    fn debug(self, context: impl Display) -> Result<T, E>;
    fn trace(self, context: impl Display) -> Result<T, E>;
}

impl<T, E> RecordErr<T, E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    #[inline]
    fn error(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            #[cfg(feature = "tracing")]
            tracing::error!("{}: {:?}", context, err);
            #[cfg(feature = "log")]
            log::error!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            #[cfg(feature = "tracing")]
            tracing::warn!("{}: {:?}", context, err);
            #[cfg(feature = "log")]
            log::warn!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn info(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            #[cfg(feature = "tracing")]
            tracing::info!("{}: {:?}", context, err);
            #[cfg(feature = "log")]
            log::info!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}: {:?}", context, err);
            #[cfg(feature = "log")]
            log::debug!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            #[cfg(feature = "tracing")]
            tracing::trace!("{}: {:?}", context, err);
            #[cfg(feature = "log")]
            log::trace!("{}: {:?}", context, err);
        }
        self
    }
}
