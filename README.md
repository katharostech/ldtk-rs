# ldtk

[![Crates.io](https://img.shields.io/crates/v/ldtk.svg)](https://crates.io/crates/ldtk)
[![Docs.rs](https://docs.rs/ldtk/badge.svg)](https://docs.rs/ldtk)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

A crate for reading the [LDtk] tile map format.

This crate implements the Rust structures necessary to Serialize and Deserialize the LDtk map
format using [`serde`].

## Example

**`Cargo.toml`:**

```toml
# Note: We must specify the version of LDtk we want to support in a feature flag
ldtk = { version = "0.4.0", features = ["ldtk-v0-9-3"] }
```

**`main.rs`:**

```rust
use ldtk::Project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the map
    let map: Project =
        serde_json::from_slice(include_bytes!("../examples/full-features.ldtk"))?;

    // Debug print the map
    dbg!(map);

    Ok(())
}
```

[ldtk]: https://github.com/deepnight/ldtk

## Extra Documentation

For more information on using the LDtk project structure inside of games see the [LDtk
docs][ldtk_docs].

## Naming Conventions

This crate uses the same struct field names as the raw JSON format in almost all cases. The
exception to this rule is for fields that are named `type`. In this case the name of the field
will be prefixed with the name of it's struct, converted to snake case, for instance
`field_def_type`.

### The `__field_name` Convention

It is a convention of the LDtk map format to prefix certain convenience fields with two
underscores, such as `__tile_src_rect`. These fields are redundant, as the information in them
is also present elsewhere in the project structure, but they are provided in some cases where it
may make traversing the project structure much more convenient.

The double underscore prefix does **not** indicate that the field is a private field or an
implementation detail.

[ldtk_docs]: https://ldtk.io/docs/game-dev/json-overview/

## Build Configuration & Features

This entire crate is automatically generated from the [JSON Schema](http://json-schema.org/)
from the [LDtk repo](https://github.com/deepnight/ldtk/blob/master/docs/JSON_SCHEMA.json) and
can be automatically updated with LDtk releases.

By default the crate requires that you pass a feature indicating a version of the JSON schema
that is built into the crate so that it doesn't require network access to build, but you can
also supply the `download-schema` cargo feature to make the crate download the JSON schema from
the LDtk repo instead ( See "Downloading the Schema" below ).

This crate currently has the schema for the following versions of LDtk built-in:

- `v0.9.3`
- `v0.8.1` ( patched, see note below )
- `v0.7.0`

The format of the feature flags for specific versions is like this: `ldtk-v0-9-3`. Note that the
periods in the version have been replaced with dashes to be conformat with cargo's feature
naming conventions.

> **Note:** In version 0.8.1 there was a field that was marked as non-null in the JSON schema,
> but in one of the LDtk sample maps the field was null. We patched the JSON schema to make the
> field nullable so that the map would load correctly.

As newer LDtk versions are released we may add new built-in schemas. These will each be under
new feature flags so that updates to the schema do not need to be breaking changes.

### Downloading the Schema

When the `download-schema` feature is provided, you can specify which version of LDtk you want
to build this crate for by setting the `LDTK_VERSION` environment variable at build time.
`LDTK_VERSION` will default to `master` which will pull the latest schema from the master branch
of the LDtk git repo.

## License

LDtk-rs is licensed under the [Katharos License][k_license] which places certain restrictions on
what you are allowed to use it for. Please read and understand the terms before using LDtk-rs
for your project.

[k_license]: https://github.com/katharostech/katharos-license

## Similar Projects & Bevy Integration

- [`bevy_ldtk`](https://github.com/katharostech/bevy_ldtk): A Bevy plugin for loading LDtk maps that uses this crate.
- [`ldtk_rust`](https://github.com/estivate/ldtk_rust): modified version of the LDtk bindings created by [QuickType](https://github.com/quicktype/quicktype), with a good Bevy example that is hopefully useful for non-Bevy game engines, too.
- Another [`bevy-ldtk`](https://github.com/tigregalis/bevy-ldtk): Work-in-progress Bevy plugin for loading LDtk maps.
