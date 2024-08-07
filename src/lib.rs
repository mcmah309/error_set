#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "tracing", feature = "log"))]
mod record;
#[cfg(any(feature = "tracing", feature = "log"))]
pub use record::*;

pub use error_set_impl::*;

pub trait CoerceResult<T, E1> {
    fn coerce<E2: From<E1>>(self) -> Result<T, E2>;
}

impl<T, E1> CoerceResult<T, E1> for Result<T, E1> {
    #[inline(always)]
    fn coerce<E2: From<E1>>(self) -> Result<T, E2> {
        self.map_err(Into::<E2>::into)
    }
}
