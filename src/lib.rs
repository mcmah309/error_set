pub use error_set_internal::*;

pub trait Coerce<T> {
    fn coerce(self) -> T;
}

impl<T, E1, E2> Coerce<Result<T, E2>> for Result<T, E1>
where
    E2: From<E1> + std::error::Error,
    E1: std::error::Error,
{
    #[inline(always)]
    fn coerce(self) -> Result<T, E2> {
        self.map_err(Into::into)
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