#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "coerce_macro", doc = "Generates a `coerce!` macro for each error set to handle coercing between intersections.")]
#![cfg_attr(feature = "tracing", doc = "Enables tracing crate methods on `Result` that are applied on `Err` - e.g. `result.warn(...)`.")]
#![cfg_attr(feature = "log", doc = "Enables logging crate methods on `Result` that are applied on `Err` - e.g. `result.warn(...)`.")]
#[cfg(all(feature = "tracing", feature = "log"))]
compile_error!("Features 'tracing' and 'log' cannot be enabled at the same time.");

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
