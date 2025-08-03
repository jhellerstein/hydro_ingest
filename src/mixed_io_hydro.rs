use hydro_lang::*;
use std::io::{self, Write};
pub fn mixed_io_hydro(process: &Process) {
    process
        .source_iter(q!(std::iter::once(())))
        .map(
            q!(
                | _ | { print!("Processing"); io::stdout().flush().unwrap(); for i in 1
                ..= 3 { std::thread::sleep(std::time::Duration::from_millis(500));
                eprint!("."); io::stderr().flush().unwrap(); print!("."); io::stdout()
                .flush().unwrap(); } println!("\nDone!");
                eprintln!("Process completed successfully"); }
            ),
        )
        .for_each(q!(| _ | {}));
}
