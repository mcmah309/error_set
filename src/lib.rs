#![doc = include_str!("../README.md")]

pub use error_set_impl::*;

pub trait Coerce<T> {
    fn coerce(self) -> T;
}

pub trait CoerceResult<T, E1> {
    fn coerce<E2: From<E1> + std::error::Error>(self) -> Result<T,E2>;
}

impl<T, E1> CoerceResult<T,E1> for Result<T, E1>
where
    E1: std::error::Error,
{
    #[inline(always)]
    fn coerce<E2: From<E1> + std::error::Error>(self) -> Result<T, E2> {
        self.map_err(Into::<E2>::into)
    }
}
pub trait ErrorSetMarker {}

impl<E1, E2> Coerce<E2> for E1
where
    E2: From<E1> + std::error::Error,
    E1: std::error::Error + ErrorSetMarker,
{
    #[inline(always)]
    fn coerce(self) -> E2 {
        self.into()
    }
}