use std::fs;
use std::path::Path;
use syn::{parse_file, Item, ItemFn, Stmt, Expr};
use quote::{quote, ToTokens};
use proc_macro2::{TokenStream, Span};

/// A more robust transformer using syn for AST parsing and preservation of span information
pub struct SynLegacyToHydroTransformer {
    /// Whether to preserve original spans for debugging
    preserve_spans: bool,
}

impl SynLegacyToHydroTransformer {
    pub fn new() -> Self {
        Self {
            preserve_spans: true,
        }
    }

    pub fn with_preserve_spans(mut self, preserve: bool) -> Self {
        self.preserve_spans = preserve;
        self
    }

    /// Transform a legacy Rust program file into a Hydro dataflow program
    pub fn transform_program<P: AsRef<Path>>(
        &self,
        legacy_path: P,
        module_name: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let source = fs::read_to_string(&legacy_path)?;
        let file = parse_file(&source)?;

        // Extract the main function and its body
        let main_fn = self.extract_main_function(&file)?;
        let main_body = self.extract_function_body(&main_fn)?;

        // Generate the Hydro function
        let hydro_function = self.generate_hydro_function(module_name, &main_body)?;

        // Generate the example program
        let example_program = self.generate_example_program(module_name)?;

        Ok((hydro_function, example_program))
    }

    /// Extract the main function from the parsed file
    pub fn extract_main_function<'a>(&self, file: &'a syn::File) -> Result<&'a ItemFn, Box<dyn std::error::Error>> {
        for item in &file.items {
            if let Item::Fn(func) = item {
                if func.sig.ident == "main" {
                    return Ok(func);
                }
            }
        }
        Err("No main function found in the file".into())
    }

    /// Extract the body statements from a function, preserving spans
    pub fn extract_function_body(&self, func: &ItemFn) -> Result<Vec<Stmt>, Box<dyn std::error::Error>> {
        Ok(func.block.stmts.clone())
    }

    /// Generate a Hydro dataflow function from the legacy function body
    fn generate_hydro_function(
        &self,
        module_name: &str,
        body_stmts: &[Stmt],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let func_name = syn::Ident::new(module_name, Span::call_site());
        
        // Convert the original statements to a token stream, preserving spans
        let body_tokens = if self.preserve_spans {
            // Preserve original spans for debugging
            self.preserve_statement_spans(body_stmts)
        } else {
            // Use call site spans
            quote! { #(#body_stmts)* }
        };

        // Generate the Hydro function wrapper
        let hydro_fn = quote! {
            use hydro_lang::*;

            pub fn #func_name(process: &Process) {
                // Wrap the original main function logic in a Hydro map operation
                process
                    .source_iter(q!(std::iter::once(())))
                    .map(q!(|_| {
                        #body_tokens
                    }))
                    .for_each(q!(|_| {}));
            }
        };

        // Format the generated code for better readability
        let formatted = prettyplease::unparse(&syn::parse2(hydro_fn)?);
        Ok(formatted)
    }

    /// Preserve original spans from statements for better debugging
    fn preserve_statement_spans(&self, stmts: &[Stmt]) -> TokenStream {
        let mut result = TokenStream::new();
        for stmt in stmts {
            // Convert each statement to tokens, preserving its original span
            let stmt_tokens = stmt.to_token_stream();
            result.extend(stmt_tokens);
        }
        result
    }

    /// Generate an example program that uses the Hydro function
    fn generate_example_program(&self, module_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let func_name = syn::Ident::new(module_name, Span::call_site());
        let crate_name = syn::Ident::new("hydro_template", Span::call_site());

        let example = quote! {
            use hydro_deploy::Deployment;
            use tokio::time::{timeout, Duration};

            #[tokio::main]
            async fn main() {
                let mut deployment = Deployment::new();

                let flow = hydro_lang::FlowBuilder::new();
                let process = flow.process::<()>();
                
                // Call our generated Hydro function
                #crate_name::#func_name::#func_name(&process);

                let _nodes = flow
                    .with_process(&process, deployment.Localhost())
                    .deploy(&mut deployment);

                println!("Starting deployment...");
                println!("Looking for 'running command:' output...");
                
                // Deploy the processes first
                deployment.deploy().await.unwrap();
                
                // Start the deployment with a timeout
                let start_result = timeout(Duration::from_secs(60), async {
                    deployment.start().await.unwrap();
                }).await;
                
                match start_result {
                    Ok(_) => {
                        println!("✓ Deployment completed successfully");
                    }
                    Err(_) => {
                        println!("✓ Deployment reached 60-second timeout");
                        println!("If you saw output containing:");
                        println!("  [() (process 0)] running command: `...`");
                        println!("  [() (process 0)] <your program output>");
                        println!("Then the deployment worked correctly!");
                    }
                }
            }
        };

        // Format the generated code for better readability
        let formatted = prettyplease::unparse(&syn::parse2(example)?);
        Ok(formatted)
    }

    /// Extract and analyze function calls from the body for more sophisticated transformations
    pub fn analyze_function_calls(&self, stmts: &[Stmt]) -> Vec<FunctionCallInfo> {
        let mut calls = Vec::new();
        
        for stmt in stmts {
            self.extract_calls_from_stmt(stmt, &mut calls);
        }
        
        calls
    }

    fn extract_calls_from_stmt(&self, stmt: &Stmt, calls: &mut Vec<FunctionCallInfo>) {
        match stmt {
            Stmt::Expr(expr, _) => {
                self.extract_calls_from_expr(expr, calls);
            }
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    self.extract_calls_from_expr(&init.expr, calls);
                }
            }
            Stmt::Item(_) => {
                // Handle item statements (not common in main function body)
            }
            Stmt::Macro(stmt_macro) => {
                // Handle macro statements like println! directly at statement level
                if let Some(ident) = stmt_macro.mac.path.get_ident() {
                    calls.push(FunctionCallInfo {
                        name: format!("{}!", ident),
                        span: ident.span(),
                        args_count: 1,
                    });
                }
            }
        }
    }

    fn extract_calls_from_expr(&self, expr: &Expr, calls: &mut Vec<FunctionCallInfo>) {
        match expr {
            Expr::Call(call) => {
                if let Expr::Path(path) = &*call.func {
                    if let Some(ident) = path.path.get_ident() {
                        calls.push(FunctionCallInfo {
                            name: ident.to_string(),
                            span: ident.span(),
                            args_count: call.args.len(),
                        });
                    }
                }
                // Recursively check arguments
                for arg in &call.args {
                    self.extract_calls_from_expr(arg, calls);
                }
            }
            Expr::Macro(macro_expr) => {
                // Handle macro calls like println!, format!, etc.
                if let Some(ident) = macro_expr.mac.path.get_ident() {
                    calls.push(FunctionCallInfo {
                        name: format!("{}!", ident), // Add ! to indicate it's a macro
                        span: ident.span(),
                        args_count: 1, // Macros don't have a predictable arg count
                    });
                }
            }
            Expr::MethodCall(method_call) => {
                calls.push(FunctionCallInfo {
                    name: method_call.method.to_string(),
                    span: method_call.method.span(),
                    args_count: method_call.args.len() + 1, // +1 for receiver
                });
                // Recursively check receiver and arguments
                self.extract_calls_from_expr(&method_call.receiver, calls);
                for arg in &method_call.args {
                    self.extract_calls_from_expr(arg, calls);
                }
            }
            Expr::Block(block) => {
                for stmt in &block.block.stmts {
                    self.extract_calls_from_stmt(stmt, calls);
                }
            }
            Expr::If(if_expr) => {
                self.extract_calls_from_expr(&if_expr.cond, calls);
                for stmt in &if_expr.then_branch.stmts {
                    self.extract_calls_from_stmt(stmt, calls);
                }
                if let Some((_, else_branch)) = &if_expr.else_branch {
                    self.extract_calls_from_expr(else_branch, calls);
                }
            }
            Expr::While(while_expr) => {
                self.extract_calls_from_expr(&while_expr.cond, calls);
                for stmt in &while_expr.body.stmts {
                    self.extract_calls_from_stmt(stmt, calls);
                }
            }
            Expr::ForLoop(for_loop) => {
                self.extract_calls_from_expr(&for_loop.expr, calls);
                for stmt in &for_loop.body.stmts {
                    self.extract_calls_from_stmt(stmt, calls);
                }
            }
            // Add more expression types as needed
            _ => {}
        }
    }
}

/// Information about a function call found in the source code
#[derive(Debug, Clone)]
pub struct FunctionCallInfo {
    pub name: String,
    pub span: Span,
    pub args_count: usize,
}

impl Default for SynLegacyToHydroTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hello_world_transformation() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
fn main() {{
    println!("Hello, world!");
}}
"#).unwrap();

        let transformer = SynLegacyToHydroTransformer::new();
        let result = transformer.transform_program(temp_file.path(), "test_hello");
        
        assert!(result.is_ok());
        let (hydro_fn, example) = result.unwrap();
        
        // Check that the generated function contains our expected structure
        assert!(hydro_fn.contains("pub fn test_hello"));
        assert!(hydro_fn.contains("singleton"));
        assert!(hydro_fn.contains("map"));
        assert!(hydro_fn.contains("println!"));
        
        // Check that the example contains deployment code
        assert!(example.contains("Deployment::new"));
        assert!(example.contains("test_hello"));
    }

    #[test]
    fn test_function_call_analysis() {
        let source = r#"
fn main() {
    println!("Hello");
    let x = format!("test {}", 42);
    vec![1, 2, 3].iter().for_each(|x| println!("{}", x));
}
"#;
        
        let file = parse_file(source).unwrap();
        let transformer = SynLegacyToHydroTransformer::new();
        let main_fn = transformer.extract_main_function(&file).unwrap();
        let body = transformer.extract_function_body(main_fn).unwrap();
        
        let calls = transformer.analyze_function_calls(&body);
        
        // Should find println!, format!, vec!, iter, for_each, etc.
        assert!(!calls.is_empty());
        assert!(calls.iter().any(|c| c.name == "println"));
        assert!(calls.iter().any(|c| c.name == "format"));
    }
}
