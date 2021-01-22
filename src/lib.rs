//! A crate for reading reading the [LDtk] 2D tile map format.
//!
//! The LDtk map format is simply a JSON format, which allows us to read the map file using
//! [`serde`]. This crate contains the serializable Rust structures corresponding to the map file
//! JSON structure.
//!
//! ## Example
//!
//! ```rust
//! use ldtk::Project;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load the map
//!     let map: Project = serde_json::from_slice(include_bytes!("../examples/full-features.ldtk"))?;
//!
//!     // Debug print the map
//!     dbg!(map);
//!
//!     Ok(())
//! }
//! ```
//!
//! [LDtk]: https://github.com/deepnight/ldtk

// The actual code is generated in the `build.rs`.
include!(concat!(env!("OUT_DIR"), "/schema.rs"));
