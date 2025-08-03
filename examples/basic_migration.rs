// Example showing how to use the LegacyToHydroTransformer
use hydro_template::transformer::LegacyToHydroTransformer;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Legacy to Hydro Migration Example");
    println!("==================================");
    
    let transformer = LegacyToHydroTransformer::new();
    let legacy_path = Path::new("src/legacy/hello_world.rs");
    let output_path = Path::new("target/hello_world_hydro.rs");
    
    // Ensure target directory exists
    std::fs::create_dir_all("target")?;
    
    println!("Transforming legacy Rust program...");
    transformer.transform_program(legacy_path, output_path)?;
    
    println!("✓ Successfully transformed {} to {}", 
             legacy_path.display(), 
             output_path.display());
    
    // Read and display the transformed code
    let transformed_code = std::fs::read_to_string(output_path)?;
    println!("\nGenerated Hydro program:");
    println!("{}", "=".repeat(40));
    println!("{}", transformed_code);
    
    Ok(())
}
