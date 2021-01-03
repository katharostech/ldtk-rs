use ldtk::Project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the map
    let map: Project = serde_json::from_slice(include_bytes!("./full-features.ldtk"))?;

    // Debug print the map
    dbg!(map);

    Ok(())
}
