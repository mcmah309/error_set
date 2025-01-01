#![cfg_attr(not(any(test, feature = "tracing", feature = "log")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "tracing", feature = "log", feature = "context_stub"))]
mod context;
#[cfg(any(feature = "tracing", feature = "log", feature = "context_stub"))]
pub use context::*;

#[cfg(feature = "defmt")]
mod defmt_context;
#[cfg(feature = "defmt")]
pub use defmt_context::*;

#[cfg(all(feature = "tracing", feature = "log"))]
compile_error!("Features 'tracing' and 'log' cannot be enabled at the same time.");
#[cfg(all(feature = "tracing", feature = "defmt"))]
compile_error!("Features 'tracing' and 'defmt' cannot be enabled at the same time.");
#[cfg(all(feature = "log", feature = "defmt"))]
compile_error!("Features 'log' and 'defmt' cannot be enabled at the same time.");
#[cfg(all(feature = "tracing", feature = "context_stub"))]
compile_error!("Features 'tracing' and 'context_stub' cannot be enabled at the same time.");
#[cfg(all(feature = "log", feature = "context_stub"))]
compile_error!("Features 'log' and 'context_stub' cannot be enabled at the same time.");
#[cfg(all(feature = "defmt", feature = "context_stub"))]
compile_error!("Features 'defmt' and 'context_stub' cannot be enabled at the same time.");

pub use declare_impl::error_set;
#[cfg(any(feature = "tracing", feature = "log", feature = "context_stub", feature = "defmt"))]
pub use err_trail::*;

pub trait CoerceResult<T, E1> {
    fn coerce<E2: From<E1>>(self) -> Result<T, E2>;
}

impl<T, E1> CoerceResult<T, E1> for Result<T, E1> {
    #[inline(always)]
    fn coerce<E2: From<E1>>(self) -> Result<T, E2> {
        self.map_err(Into::<E2>::into)
    }
}
