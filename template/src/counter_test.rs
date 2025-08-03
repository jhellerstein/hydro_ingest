use hydro_lang::*;

pub fn counter_test(process: &Process) {
    process
        .source_iter(q!(std::iter::once(())))
        .map(q!(|_| {
            // Legacy main function body wrapped in Hydro map operator
                for i in 1..=5 {
                    println!("Count: {}", i);
                }
        }))
        .for_each(q!(|_| {}));
}