// Example showing how to use the IOToHydroTransformer for I/O-aware migration
use hydro_template::io_transformer::IOToHydroTransformer;
use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("I/O-Aware Legacy to Hydro Migration Example");
    println!("===========================================");
    
    let transformer = IOToHydroTransformer::new()
        .with_preserve_spans(true); // Enable span preservation for debugging
    
    // Test with interactive hello program
    let interactive_path = Path::new("src/legacy/interactive_hello.rs");
    println!("\nTransforming interactive hello program...");
    
    let (hydro_function, example_program) = transformer.transform_program(interactive_path, "interactive_hello_hydro")?;
    
    // Analyze I/O operations
    let source = fs::read_to_string(interactive_path)?;
    let file = syn::parse_file(&source)?;
    let main_fn = transformer.extract_main_function(&file)?;
    let body = transformer.extract_function_body(main_fn)?;
    let io_operations = transformer.analyze_io_operations(&body);
    
    println!("Found {} I/O operations in the legacy code:", io_operations.len());
    for op in &io_operations {
        println!("  - {:?}", op.operation_type);
    }
    
    // Write the generated files
    let hydro_module_path = Path::new("src/interactive_hello_hydro.rs");
    fs::write(hydro_module_path, &hydro_function)?;
    
    let example_path = Path::new("examples/interactive_hello_hydro.rs");
    fs::write(example_path, &example_program)?;
    
    println!("✓ Successfully transformed {} to I/O-aware Hydro dataflow:", 
             interactive_path.display());
    println!("  - Hydro function: {}", hydro_module_path.display());
    println!("  - Example program: {}", example_path.display());
    
    // Test with echo lines program
    println!("\n{}", "=".repeat(50));
    let echo_path = Path::new("src/legacy/echo_lines.rs");
    println!("Transforming echo lines program...");
    
    let (hydro_function2, example_program2) = transformer.transform_program(echo_path, "echo_lines_hydro")?;
    
    // Analyze I/O operations for echo program
    let source2 = fs::read_to_string(echo_path)?;
    let file2 = syn::parse_file(&source2)?;
    let main_fn2 = transformer.extract_main_function(&file2)?;
    let body2 = transformer.extract_function_body(main_fn2)?;
    let io_operations2 = transformer.analyze_io_operations(&body2);
    
    println!("Found {} I/O operations in echo program:", io_operations2.len());
    for op in &io_operations2 {
        println!("  - {:?}", op.operation_type);
    }
    
    // Write the generated files
    let hydro_module_path2 = Path::new("src/echo_lines_hydro.rs");
    fs::write(hydro_module_path2, &hydro_function2)?;
    
    let example_path2 = Path::new("examples/echo_lines_hydro.rs");
    fs::write(example_path2, &example_program2)?;
    
    println!("✓ Successfully transformed {} to I/O-aware Hydro dataflow:", 
             echo_path.display());
    println!("  - Hydro function: {}", hydro_module_path2.display());
    println!("  - Example program: {}", example_path2.display());
    
    // Test with mixed I/O program
    println!("\n{}", "=".repeat(50));
    let mixed_path = Path::new("src/legacy/mixed_io.rs");
    println!("Transforming mixed I/O program...");
    
    let (hydro_function3, example_program3) = transformer.transform_program(mixed_path, "mixed_io_hydro")?;
    
    // Analyze I/O operations for mixed program
    let source3 = fs::read_to_string(mixed_path)?;
    let file3 = syn::parse_file(&source3)?;
    let main_fn3 = transformer.extract_main_function(&file3)?;
    let body3 = transformer.extract_function_body(main_fn3)?;
    let io_operations3 = transformer.analyze_io_operations(&body3);
    
    println!("Found {} I/O operations in mixed I/O program:", io_operations3.len());
    for op in &io_operations3 {
        println!("  - {:?}", op.operation_type);
    }
    
    // Write the generated files
    let hydro_module_path3 = Path::new("src/mixed_io_hydro.rs");
    fs::write(hydro_module_path3, &hydro_function3)?;
    
    let example_path3 = Path::new("examples/mixed_io_hydro.rs");
    fs::write(example_path3, &example_program3)?;
    
    println!("✓ Successfully transformed {} to I/O-aware Hydro dataflow:", 
             mixed_path.display());
    println!("  - Hydro function: {}", hydro_module_path3.display());
    println!("  - Example program: {}", example_path3.display());
    
    // Display the generated Hydro function for interactive hello
    println!("\n{}", "=".repeat(60));
    println!("Generated I/O-aware Hydro function (interactive_hello):");
    println!("{}", "-".repeat(60));
    println!("{}", hydro_function);
    
    println!("\nTo run the generated I/O-aware programs:");
    println!("  cargo run --example interactive_hello_hydro");
    println!("  cargo run --example echo_lines_hydro");
    println!("  cargo run --example mixed_io_hydro");
    
    Ok(())
}
