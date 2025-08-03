// Example showing how to use the LegacyToHydroTransformer
use hydro_template::transformer::LegacyToHydroTransformer;
use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Legacy to Hydro Migration Example");
    println!("==================================");
    
    let transformer = LegacyToHydroTransformer::new();
    let legacy_path = Path::new("src/legacy/hello_world.rs");
    
    println!("Transforming legacy Rust program...");
    let (hydro_function, example_program) = transformer.transform_program(legacy_path, "legacy_hello_world")?;
    
    // Write the generated Hydro function
    let hydro_module_path = Path::new("src/legacy_hello_world.rs");
    fs::write(hydro_module_path, &hydro_function)?;
    
    // Write the example program
    let example_path = Path::new("examples/legacy_hello_world.rs");
    fs::write(example_path, &example_program)?;
    
    println!("✓ Successfully transformed {} to Hydro dataflow:", 
             legacy_path.display());
    println!("  - Hydro function: {}", hydro_module_path.display());
    println!("  - Example program: {}", example_path.display());
    
    // Display the generated Hydro function
    println!("\nGenerated Hydro function:");
    println!("{}", "-".repeat(50));
    println!("{}", hydro_function);
    
    println!("\nTo run the generated program:");
    println!("  cargo run --example legacy_hello_world");
    
    Ok(())
}
