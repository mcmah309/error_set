#![cfg_attr(not(any(test, feature = "tracing", feature = "log")), no_std)]

#[cfg(any(feature = "tracing", feature = "log", feature = "stub"))]
mod tracing_log_stub;
#[cfg(any(feature = "tracing", feature = "log", feature = "stub"))]
pub use  tracing_log_stub::*;

#[cfg(feature = "defmt")]
mod defmt;
#[cfg(feature = "defmt")]
pub use defmt::*;


#[cfg(all(feature = "tracing", feature = "defmt"))]
compile_error!("Features 'tracing' and 'defmt' cannot be enabled at the same time.");
#[cfg(all(feature = "log", feature = "defmt"))]
compile_error!("Features 'log' and 'defmt' cannot be enabled at the same time.");
#[cfg(all(feature = "defmt", feature = "stub"))]
compile_error!("Features 'defmt' and 'context_stub' cannot be enabled at the same time.");