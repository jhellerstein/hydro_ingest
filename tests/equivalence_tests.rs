use std::process::Command;
use std::fs;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_hello_world_equivalence() {
    // Run the original legacy program
    let legacy_output = run_legacy_program("generator/legacy_programs/hello_world.rs")
        .expect("Failed to run legacy program");
    
    // Generate the Hydro version using our generator
    generate_hydro_program("legacy_programs/hello_world.rs", "hello_world_test")
        .expect("Failed to generate Hydro program");
    
    // Run the generated Hydro program and capture output
    let hydro_output = run_generated_hydro_program("hello_world_test").await
        .expect("Failed to run Hydro program");
    
    // Compare outputs
    println!("Legacy output: {:?}", legacy_output);
    println!("Hydro output: {:?}", hydro_output);
    
    // The hydro output will have deployment info, so we need to extract just the program output
    let hydro_program_output = extract_program_output_from_hydro(&hydro_output);
    
    assert_eq!(legacy_output.trim(), hydro_program_output.trim(), 
               "Legacy and Hydro programs should produce identical output");
}

#[tokio::test]
async fn test_counter_equivalence() {
    // Run the original legacy program
    let legacy_output = run_legacy_program("generator/legacy_programs/counter.rs")
        .expect("Failed to run legacy program");
    
    // Generate the Hydro version using our generator
    generate_hydro_program("legacy_programs/counter.rs", "counter_test")
        .expect("Failed to generate Hydro program");
    
    // Run the generated Hydro program and capture output
    let hydro_output = run_generated_hydro_program("counter_test").await
        .expect("Failed to run Hydro program");
    
    // Compare outputs
    println!("Legacy output: {:?}", legacy_output);
    println!("Hydro output: {:?}", hydro_output);
    
    // The hydro output will have deployment info, so we need to extract just the program output
    let hydro_program_output = extract_program_output_from_hydro(&hydro_output);
    
    assert_eq!(legacy_output.trim(), hydro_program_output.trim(), 
               "Legacy and Hydro programs should produce identical output");
}

fn run_legacy_program(program_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary executable name
    let exe_name = format!("temp_legacy_{}", std::process::id());
    
    // Compile the program
    let compile_result = Command::new("rustc")
        .arg(program_path)
        .arg("-o")
        .arg(&exe_name)
        .output()?;
    
    if !compile_result.status.success() {
        return Err(format!("Legacy compilation failed: {}", 
                          String::from_utf8_lossy(&compile_result.stderr)).into());
    }
    
    // Run the compiled program
    let run_result = Command::new(format!("./{}", exe_name))
        .output()?;
    
    // Clean up executable
    fs::remove_file(&exe_name).ok();
    
    if run_result.status.success() {
        Ok(String::from_utf8_lossy(&run_result.stdout).to_string())
    } else {
        Err(format!("Legacy program execution failed: {}", 
                   String::from_utf8_lossy(&run_result.stderr)).into())
    }
}

fn generate_hydro_program(legacy_path: &str, module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Run our generator to create the Hydro version
    let generate_result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(legacy_path)
        .arg(module_name)
        .current_dir("generator")
        .output()?;
    
    if !generate_result.status.success() {
        return Err(format!("Generator failed: {}", 
                          String::from_utf8_lossy(&generate_result.stderr)).into());
    }
    
    Ok(())
}

async fn run_generated_hydro_program(module_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Run the generated Hydro program with timeout
    let result = timeout(Duration::from_secs(90), async {
        let run_result = Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(module_name)
            .current_dir("template")
            .output()?;
        
        if run_result.status.success() {
            Ok::<String, Box<dyn std::error::Error>>(String::from_utf8_lossy(&run_result.stdout).to_string())
        } else {
            Err(format!("Hydro program execution failed: {}", 
                       String::from_utf8_lossy(&run_result.stderr)).into())
        }
    }).await??;
    
    Ok(result)
}

fn extract_program_output_from_hydro(hydro_output: &str) -> String {
    // Look for lines that contain the actual program output
    // Format: [() (process 0)] Hello, world!
    let mut program_lines = Vec::new();
    
    for line in hydro_output.lines() {
        if line.contains("[() (process 0)]") && !line.contains("running command:") {
            // Extract just the program output after the process identifier
            if let Some(output_start) = line.find("] ") {
                let program_output = &line[output_start + 2..];
                program_lines.push(program_output);
            }
        }
    }
    
    program_lines.join("\n")
}
