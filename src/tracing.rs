use std::fmt::Display;

use tracing::{debug, error, info, trace, warn};

pub trait LogErr<T, E> {
    fn log_e(self, context: impl Display) -> Result<T, E>;
    fn log_w(self, context: impl Display) -> Result<T, E>;
    fn log_i(self, context: impl Display) -> Result<T, E>;
    fn log_d(self, context: impl Display) -> Result<T, E>;
    fn log_t(self, context: impl Display) -> Result<T, E>;
}

impl<T, E> LogErr<T, E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    #[inline]
    fn log_e(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            error!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn log_w(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            warn!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn log_i(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            info!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn log_d(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            debug!("{}: {:?}", context, err);
        }
        self
    }

    #[inline]
    fn log_t(self, context: impl Display) -> Result<T, E> {
        if let Err(ref err) = self {
            trace!("{}: {:?}", context, err);
        }
        self
    }
}
