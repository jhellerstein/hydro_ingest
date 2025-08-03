// Example showing how to use the SynLegacyToHydroTransformer
use hydro_template::syn_transformer::SynLegacyToHydroTransformer;
use std::path::Path;
use std::fs;
use syn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Legacy to Hydro Migration Example (with syn)");
    println!("==============================================");
    
    let transformer = SynLegacyToHydroTransformer::new()
        .with_preserve_spans(true); // Enable span preservation for debugging
    
    let legacy_path = Path::new("src/legacy/hello_world.rs");
    
    println!("Transforming legacy Rust program with AST parsing...");
    let (hydro_function, example_program) = transformer.transform_program(legacy_path, "syn_hello_world")?;
    
    // Analyze function calls in the legacy code
    let source = fs::read_to_string(legacy_path)?;
    let file = syn::parse_file(&source)?;
    let main_fn = transformer.extract_main_function(&file)?;
    let body = transformer.extract_function_body(main_fn)?;
    let function_calls = transformer.analyze_function_calls(&body);
    
    println!("Found {} function calls in the legacy code:", function_calls.len());
    for call in &function_calls {
        println!("  - {} (with {} args)", call.name, call.args_count);
    }
    
    // Write the generated Hydro function
    let hydro_module_path = Path::new("src/syn_hello_world.rs");
    fs::write(hydro_module_path, &hydro_function)?;
    
    // Write the example program
    let example_path = Path::new("examples/syn_hello_world.rs");
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
    println!("  cargo run --example syn_hello_world");
    
    Ok(())
}
