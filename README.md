# LDtk-rs

A crate for reading reading the [LDtk] 2D tile map format.

The LDtk map format is simply a JSON format, which allows us to read the map file using
[`serde`]. This crate contains the serializable Rust structures corresponding to the map file
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

## License

LDtk-rs is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use it for. Please read and understand the terms before using LDtk-rs for your project.

[k_license]: https://github.com/katharostech/katharos-license
