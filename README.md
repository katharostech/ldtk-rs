# LDtk-rs

A crate for reading reading the [LDtk] 2D tile map format.

The LDtk map format is simply a JSON format, which allows us to read the map file using
`serde`. This crate contains the serializable Rust structures corresponding to the map file
JSON structure.

## Example

```rust
use ldtk::Project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the map
    let map: Project = serde_json::from_slice(include_bytes!("../examples/full-features.ldtk"))?;

    // Debug print the map
    dbg!(map);

    Ok(())
}
```

[ldtk]: https://github.com/deepnight/ldtk

The entire crate is automatically generated from the [JSON Schema](http://json-schema.org/) in the [LDtk repo](https://github.com/deepnight/ldtk/blob/master/docs/JSON_SCHEMA.json).

## Build Configuration & Features

By default, the crate will generate code for reading the LDtk format from a built-in JSON Schema file, but you can also specify any version of LDtk newer than `v0.7.0`, or `master` in the `LDTK_VERSION` environment variable and enable the `download-schema` cargo feature at build time to have the crate automatically download the latest schema and build against that.

If you do not use the `download-schema` feature, you can still override the built-in schema to use. The currently available built-in schema versions are:

- `v0.7.0`

We will try to update the built-in schemas from time to time, and the default setting will always be to use the latest built-in schema.

## Similar Projects

- [`bevy_ldtk`](https://github.com/katharostech/bevy_ldtk): A work-in-progress Bevy plugin for loading LDtk maps using this crate.
- [`ldtk_rust`](https://github.com/estivate/ldtk_rust): modified version of the LDtk bindings created by [QuickType](https://github.com/quicktype/quicktype), with a good Bevy example that is hopefully useful for non-Bevy game engines, too.
- Another [`bevy-ldtk`](https://github.com/tigregalis/bevy-ldtk): Work-in-progress Bevy plugin for loading LDtk maps.

## License

LDtk-rs is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use it for. Please read and understand the terms before using LDtk-rs for your project.

[k_license]: https://github.com/katharostech/katharos-license
