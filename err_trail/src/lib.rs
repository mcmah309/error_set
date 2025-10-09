#![cfg_attr(not(test), no_std)]

#[cfg(any(feature = "tracing", feature = "log", feature = "stub"))]
mod tracing_log_stub;
#[cfg(any(feature = "tracing", feature = "log", feature = "stub"))]
pub use tracing_log_stub::*;

#[cfg(feature = "defmt")]
mod defmt;
#[cfg(feature = "defmt")]
pub use defmt::*;
