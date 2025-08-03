use hydro_lang::*;

pub fn hello_world_test(process: &Process) {
    process
        .source_iter(q!(std::iter::once(())))
        .map(q!(|_| {
            // Legacy main function body wrapped in Hydro map operator
                println!("Hello, world!");
        }))
        .for_each(q!(|_| {}));
}