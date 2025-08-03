use std::fs;
use std::path::Path;
use regex::Regex;
use clap::{Arg, Command};

pub struct LegacyToHydroTransformer;

impl LegacyToHydroTransformer {
    pub fn new() -> Self {
        Self
    }

    pub fn transform_program(&self, input_path: &Path, output_name: &str, template_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let legacy_code = fs::read_to_string(input_path)?;
        let main_body = self.extract_main_body(&legacy_code)?;
        
        let hydro_function = self.generate_hydro_function(&main_body, output_name)?;
        let example_program = self.generate_example_program(output_name)?;
        
        // Write to template directory
        let hydro_module_path = template_dir.join("src").join(format!("{}.rs", output_name));
        fs::write(&hydro_module_path, &hydro_function)?;
        
        let example_path = template_dir.join("examples").join(format!("{}.rs", output_name));
        fs::write(&example_path, &example_program)?;
        
        // Update lib.rs to include the new module
        self.update_lib_rs(template_dir, output_name)?;
        
        println!("✓ Generated Hydro program:");
        println!("  - Module: {}", hydro_module_path.display());
        println!("  - Example: {}", example_path.display());
        println!("\nTo run: cd {} && cargo run --example {}", template_dir.display(), output_name);
        
        Ok(())
    }

    fn generate_hydro_function(&self, main_body: &str, function_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let hydro_function = format!(
r#"use hydro_lang::*;

pub fn {}(process: &Process) {{
    process
        .source_iter(q!(std::iter::once(())))
        .map(q!(|_| {{
            // Legacy main function body wrapped in Hydro map operator
{}
        }}))
        .for_each(q!(|_| {{}}));
}}"#, 
            function_name,
            self.indent_code(main_body, 12)
        );
        
        Ok(hydro_function)
    }

    fn generate_example_program(&self, function_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Read the template file
        let template_path = Path::new("../template/examples/generated_example.rs.template");
        let template_content = fs::read_to_string(template_path)?;
        
        // Replace the placeholder with the actual function call
        let function_call = format!("hydro_template::{}::{}(&process);", function_name, function_name);
        let example = template_content.replace("// GENERATED_FUNCTION_CALL_PLACEHOLDER", &function_call);
        
        Ok(example)
    }

    fn update_lib_rs(&self, template_dir: &Path, module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lib_rs_path = template_dir.join("src").join("lib.rs");
        let content = if lib_rs_path.exists() {
            fs::read_to_string(&lib_rs_path)?
        } else {
            "stageleft::stageleft_no_entry_crate!();\n\n".to_string()
        };
        
        // Check if module is already declared
        if !content.contains(&format!("pub mod {};", module_name)) {
            let new_content = format!("{}pub mod {};\n", content, module_name);
            fs::write(&lib_rs_path, new_content)?;
        }
        
        Ok(())
    }

    fn extract_main_body(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Find the main function and extract its body
        let lines: Vec<&str> = code.lines().collect();
        let mut in_main = false;
        let mut brace_count = 0;
        let mut main_body_lines = Vec::new();
        let mut seen_opening_brace = false;
        
        for line in lines {
            if line.trim().starts_with("fn main(") {
                in_main = true;
                // Check if opening brace is on the same line
                if line.contains('{') {
                    seen_opening_brace = true;
                    brace_count = 1;
                }
                continue;
            }
            
            if in_main {
                let open_braces = line.matches('{').count();
                let close_braces = line.matches('}').count();
                
                if !seen_opening_brace && open_braces > 0 {
                    seen_opening_brace = true;
                    brace_count = open_braces;
                } else {
                    brace_count += open_braces;
                }
                
                // Only subtract after we've seen the opening brace
                if seen_opening_brace {
                    brace_count = brace_count.saturating_sub(close_braces);
                    
                    if brace_count == 0 {
                        // End of main function
                        break;
                    }
                    
                    // Add line to body if we're inside the function (but not the closing line)
                    if close_braces == 0 || brace_count > 0 {
                        main_body_lines.push(line);
                    }
                }
            }
        }
        
        if main_body_lines.is_empty() {
            return Err("Could not find main function body in legacy code".into());
        }
        
        Ok(main_body_lines.join("\n"))
    }

    fn indent_code(&self, code: &str, spaces: usize) -> String {
        let indent = " ".repeat(spaces);
        code.lines()
            .map(|line| if line.trim().is_empty() { 
                line.to_string() 
            } else { 
                format!("{}{}", indent, line) 
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Hydro Ingest Generator")
        .about("Generates Hydro dataflow programs from legacy Rust code")
        .arg(Arg::new("input")
            .help("Input legacy Rust file")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .help("Output function name")
            .required(true)
            .index(2))
        .arg(Arg::new("template")
            .help("Template directory path")
            .short('t')
            .long("template")
            .default_value("../template"))
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_name = matches.get_one::<String>("output").unwrap();
    let template_dir = matches.get_one::<String>("template").unwrap();

    println!("Hydro Ingest Generator");
    println!("=====================");
    println!("Input: {}", input_file);
    println!("Output: {}", output_name);
    println!("Template: {}", template_dir);
    println!();

    let transformer = LegacyToHydroTransformer::new();
    transformer.transform_program(
        Path::new(input_file),
        output_name,
        Path::new(template_dir)
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_hello_world_output_equivalence() {
        // Create a temporary directory for this test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();
        
        // Copy the template to temp directory
        let template_source = "../template";
        let template_dest = temp_path.join("template");
        copy_dir_recursive(template_source, &template_dest).expect("Failed to copy template");
        
        // Run the original hello_world program and capture output
        let original_output = run_original_program("legacy_programs/hello_world.rs")
            .await
            .expect("Failed to run original program");
        
        // Generate Hydro version
        let transformer = LegacyToHydroTransformer::new();
        transformer.transform_program(
            Path::new("legacy_programs/hello_world.rs"),
            "hello_world_test",
            &template_dest
        ).expect("Failed to transform program");
        
        // Run the generated Hydro program and capture output
        let hydro_output = run_hydro_program(&template_dest, "hello_world_test")
            .await
            .expect("Failed to run Hydro program");
        
        // Compare outputs
        assert_eq!(
            extract_program_output(&original_output),
            extract_hydro_output(&hydro_output),
            "Original and Hydro outputs should match"
        );
    }
    
    async fn run_original_program(program_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Compile and run the original Rust program
        let output = Command::new("rustc")
            .args(&[program_path, "-o", "/tmp/original_program"])
            .output()?;
        
        if !output.status.success() {
            return Err(format!("Failed to compile original program: {}", 
                String::from_utf8_lossy(&output.stderr)).into());
        }
        
        let output = Command::new("/tmp/original_program")
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    async fn run_hydro_program(template_dir: &Path, program_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Run the generated Hydro program using cargo with a timeout
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            tokio::process::Command::new("cargo")
                .args(&["run", "--example", program_name])
                .current_dir(template_dir)
                .output()
        ).await??;
        
        if !output.status.success() {
            return Err(format!("Failed to run Hydro program: {}", 
                String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    fn extract_program_output(output: &str) -> String {
        // For simple programs, the output is just what they print
        output.trim().to_string()
    }
    
    fn extract_hydro_output(output: &str) -> String {
        // Extract the actual program output from Hydro deployment output
        // Look for lines like "[() (process 0)] Hello, world!"
        output.lines()
            .filter_map(|line| {
                if line.contains("[() (process 0)]") {
                    // Extract everything after the process prefix
                    line.split("] ").nth(1).map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
        let src = src.as_ref();
        let dst = dst.as_ref();
        
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dst.join(entry.file_name());
            
            if path.is_dir() {
                copy_dir_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }
        
        Ok(())
    }
}
