// cargo run -- --bg_name="Rust Programming Language"
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::env;

fn main() -> std::io::Result<()> {
    // get env variable
    let args: Vec<String> = env::args().collect();

    let bg_name = &args[1];
    let env = &args[2];
    
    // Step 1: Create a HashMap
    let mut my_map = HashMap::new();
    my_map.insert("name", "Rust");
    my_map.insert("type", "Programming Language");
    my_map.insert("year", "2010");
    my_map.insert("bg_name", bg_name);
    my_map.insert("env", env);

    // Step 2: Serialize the HashMap to a YAML string
    let yaml_string = serde_yaml::to_string(&my_map).expect("Failed to serialize HashMap to YAML");

    // Step 3: Write the YAML string to a file
    let mut file = File::create("output.yaml")?;
    file.write_all(yaml_string.as_bytes())?;

    println!("HashMap has been written to output.yaml");
    Ok(())
}
