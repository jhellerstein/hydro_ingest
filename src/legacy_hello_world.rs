use hydro_lang::*;

pub fn legacy_hello_world(process: &Process) {
    process
        .source_iter(q!(std::iter::once(())))
        .map(q!(|_| {
            // Legacy main function body wrapped in Hydro map operator
                println!("Hello, world!");
        }))
        .for_each(q!(|_| {}));
}