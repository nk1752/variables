use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Step 1: Create a HashMap
    let mut my_map = HashMap::new();
    my_map.insert("name", "Rust");
    my_map.insert("type", "Programming Language");
    my_map.insert("year", "2010");

    // Step 2: Serialize the HashMap to a YAML string
    let yaml_string = serde_yaml::to_string(&my_map).expect("Failed to serialize HashMap to YAML");

    // Step 3: Write the YAML string to a file
    let mut file = File::create("output.yaml")?;
    file.write_all(yaml_string.as_bytes())?;

    println!("HashMap has been written to output.yaml");
    Ok(())
}
