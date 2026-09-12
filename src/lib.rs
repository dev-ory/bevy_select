pub mod core;

#[cfg(feature = "linear")]
pub mod linear;
#[cfg(feature = "neighbor")]
pub mod neighbor;
#[cfg(feature = "planar")]
pub mod planar;
#[cfg(feature = "spatial")]
pub mod spatial;

pub mod prelude;

mod plugin;
pub use plugin::*;
