# Hydro Ingest Project

This project transforms legacy Rust programs into Hydro dataflow programs.

## Project Structure

```
hydro_ingest/
├── generator/              # Generator tool (outside template)
│   ├── src/main.rs        # Transformation logic
│   ├── legacy_programs/   # Sample legacy programs to transform
│   └── Cargo.toml         # Generator dependencies
└── template/              # Clean Hydro template project
    ├── src/lib.rs         # Template library
    ├── examples/          # Generated examples go here
    ├── Cargo.toml         # Hydro dependencies
    └── build.rs           # Stageleft build script
```

## Usage

### 1. Generate Hydro programs from legacy code

From the generator directory:

```bash
cd generator
cargo run -- legacy_programs/hello_world.rs hello_world_hydro
```

This will:
- Read the legacy program `hello_world.rs`
- Generate a Hydro function `hello_world_hydro` 
- Write files to `../template/src/hello_world_hydro.rs` and `../template/examples/hello_world_hydro.rs`
- Update `../template/src/lib.rs` to include the new module

### 2. Run the generated Hydro program

From the template directory:

```bash
cd ../template
cargo run --example hello_world_hydro
```

## Transformation Process

The generator:
1. **Extracts** the main function body from legacy Rust code
2. **Wraps** it in a Hydro `map` operator within a dataflow
3. **Generates** both a module and runnable example
4. **Adds** a 10-second timeout for automatic termination

The resulting Hydro program has identical observable behavior to the original legacy program.

## Examples

- `hello_world.rs` → Simple println transformation
- `counter.rs` → For loop transformation

Both legacy and Hydro versions produce the same output, demonstrating successful ingestion into the Hydro dataflow model.
