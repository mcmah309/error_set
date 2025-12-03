#![cfg_attr(not(any(test, feature = "combine_parts")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "combine_parts")]
mod combine_parts;
#[cfg(feature = "combine_parts")]
pub use combine_parts::combine_error_set_parts;

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
