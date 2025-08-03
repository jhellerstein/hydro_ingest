use std::fs;
use std::path::Path;

pub struct LegacyToHydroTransformer;

impl LegacyToHydroTransformer {
    pub fn new() -> Self {
        Self
    }

    pub fn transform_program(&self, input_path: &Path, output_name: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        let legacy_code = fs::read_to_string(input_path)?;
        let main_body = self.extract_main_body(&legacy_code)?;
        
        let hydro_function = self.generate_hydro_function(&main_body, output_name)?;
        let example_program = self.generate_example_program(output_name)?;
        
        Ok((hydro_function, example_program))
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
        let example = format!(
r#"use hydro_deploy::Deployment;
use tokio::time::{{timeout, Duration}};

#[tokio::main]
async fn main() {{
    let mut deployment = Deployment::new();

    let flow = hydro_lang::FlowBuilder::new();
    let process = flow.process();
    hydro_template::{}::{}(&process);

    let _nodes = flow
        .with_process(&process, deployment.Localhost())
        .deploy(&mut deployment);

    // Run for 10 seconds then exit
    match timeout(Duration::from_secs(10), deployment.run_ctrl_c()).await {{
        Ok(_) => println!("Program completed normally"),
        Err(_) => println!("Program timed out after 10 seconds"),
    }}
}}"#, 
            function_name, function_name
        );
        
        Ok(example)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_transform_hello_world() {
        let transformer = LegacyToHydroTransformer::new();
        let legacy_path = Path::new("src/legacy/hello_world.rs");
        
        let result = transformer.transform_program(legacy_path, "hello_world_hydro");
        assert!(result.is_ok(), "Transform should succeed");
        
        let (hydro_function, example_program) = result.unwrap();
        
        // Check that the hydro function contains expected structure
        assert!(hydro_function.contains("use hydro_lang::*"));
        assert!(hydro_function.contains("pub fn hello_world_hydro"));
        assert!(hydro_function.contains("source_iter"));
        assert!(hydro_function.contains("map"));
        assert!(hydro_function.contains("for_each"));
        assert!(hydro_function.contains("println!(\"Hello, world!\")"));
        
        // Check that the example program has the right structure
        assert!(example_program.contains("use hydro_deploy::Deployment"));
        assert!(example_program.contains("hydro_template::hello_world_hydro::hello_world_hydro"));
    }

    #[test]
    fn test_transform_counter() {
        let transformer = LegacyToHydroTransformer::new();
        let legacy_path = Path::new("src/legacy/counter.rs");
        
        let result = transformer.transform_program(legacy_path, "counter_hydro");
        assert!(result.is_ok(), "Transform should succeed");
        
        let (hydro_function, _) = result.unwrap();
        
        // Check that it wrapped the for loop in a map operator
        assert!(hydro_function.contains("map"));
        assert!(hydro_function.contains("for i in 1..=5"));
        assert!(hydro_function.contains("source_iter"));
    }

    #[test]
    fn test_extract_main_body() {
        let transformer = LegacyToHydroTransformer::new();
        let code = r#"fn main() {
    println!("Hello, world!");
    let x = 42;
}"#;
        
        let body = transformer.extract_main_body(code).unwrap();
        assert!(body.contains("println!(\"Hello, world!\")"));
        assert!(body.contains("let x = 42;"));
    }
}
