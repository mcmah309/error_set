use std::fmt::Display;

use tracing::{debug, error, info, trace, warn};

pub trait LogErr<T, E> {
    fn error(self, context: impl Display) -> Result<T, E>;
    fn warn(self, context: impl Display) -> Result<T, E>;
    fn info(self, context: impl Display) -> Result<T, E>;
    fn debug(self, context: impl Display) -> Result<T, E>;
    fn trace(self, context: impl Display) -> Result<T, E>;
}

impl<T, E> LogErr<T, E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    #[inline]
    fn error(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            error!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn warn(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            warn!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn info(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            info!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn debug(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            debug!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn trace(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            trace!("{}: {:?}", context, err);
        }
        self
    }
}
