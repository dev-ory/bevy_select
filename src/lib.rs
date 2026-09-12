pub mod core;

#[cfg(feature = "neighbor")]
pub mod neighbor;

#[cfg(feature = "linear")]
pub mod linear;

#[cfg(feature = "planar")]
pub mod planar;

#[cfg(feature = "spatial")]
pub mod spatial;

mod plugin;
pub use plugin::*;
