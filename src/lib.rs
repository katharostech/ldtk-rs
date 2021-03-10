//! A crate for reading the [LDtk] tile map format.
//!
//! This crate implements the Rust structures necessary to Serialize and Deserialize the LDtk map
//! format using [`serde`].
//!
//! # Example
//!
//! ```rust
//! use ldtk::Project;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load the map
//!     let map: Project =
//!         serde_json::from_slice(include_bytes!("../examples/full-features.ldtk"))?;
//!
//!     // Debug print the map
//!     dbg!(map);
//!
//!     Ok(())
//! }
//! ```
//!
//! [ldtk]: https://github.com/deepnight/ldtk
//!
//! # Extra Documentation
//!
//! For more information on using the LDtk project structure inside of games see the [LDtk
//! docs][ldtk_docs].
//!
//! # Naming Conventions
//!
//! This crate uses the same struct field names as the raw JSON format in almost all cases. The
//! exception to this rule is for fields that are named `type`. In this case the name of the field
//! will be prefixed with the name of it's struct, converted to snake case, for instance
//! `field_def_type`.
//!
//! ## The `__field_name` Convention
//!
//! It is a convention of the LDtk map format to prefix certain convenience fields with two
//! underscores, such as `__tile_src_rect`. These fields are redundant, as the information in them
//! is also present elsewhere in the project structure, but they are provided in some cases where it
//! may make traversing the project structure much more convenient.
//!
//! The double underscore prefix does **not** indicate that the field is a private field or an
//! implementation detail.
//!
//! [ldtk_docs]: https://ldtk.io/docs/game-dev/json-overview/
//!
//! # Build Configuration & Features
//!
//! This entire crate is automatically generated from the [JSON Schema](http://json-schema.org/)
//! from the [LDtk repo](https://github.com/deepnight/ldtk/blob/master/docs/JSON_SCHEMA.json) and
//! can be automatically updated with LDtk releases.
//!
//! By default the crate will use a copy of the JSON schema that is built into the crate so that it
//! doesn't require network access to build, but you can supply the `download-schema` cargo feature
//! to make the crate download the JSON schema from the LDtk repo.
//!
//! This crate currently has the schema for the following versions of LDtk built-in:
//!
//! - `v0.8.1`
//! - `v0.7.0`
//!
//! As newer LDtk versions are released we may update add new built-in schemas and update the
//! default schema to the latest one. Updates to the default schema are a breaking change and a new
//! release of this crate will be made.
//!
//! ## `LDTK_VERSION`
//!
//! You can specify which version of LDtk you want to build this crate for by setting the
//! `LDTK_VERSION` environment variable at build time. Without the `download-schema` feature, the
//! `LDTK_VERSION` will default to the latest built-in schema version.
//!
//! When the `download-schema` feature is set, the `LDTK_VERSION` will default to `master` which
//! will pull the latest schema from the master branch of the LDtk git repo.
//!
//! # License
//!
//! LDtk-rs is licensed under the [Katharos License][k_license] which places certain restrictions on
//! what you are allowed to use it for. Please read and understand the terms before using LDtk-rs
//! for your project.
//!
//! [k_license]: https://github.com/katharostech/katharos-license

// The actual code is generated in the `build.rs`.
include!(concat!(env!("OUT_DIR"), "/schema.rs"));
