//! Crop region geometry and drag handles, independent of workspace serialization.
//!
//! Applications retain their crop regions, aspect constraints and document semantics.
//! [`crop_rect`] computes the final crop rectangle, and [`CropHandle`] reports an interaction
//! without mutating that application state.

mod geometry;
mod handle;

pub use geometry::{CropAnchor, CropMode, CropRect};
pub use handle::{CropHandle, CropHandleStyle, CropResponse};

#[cfg(test)]
mod tests;