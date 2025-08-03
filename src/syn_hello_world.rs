use hydro_lang::*;
pub fn syn_hello_world(process: &Process) {
    process
        .source_iter(q!(std::iter::once(())))
        .map(q!(| _ | { println!("Hello, world!"); }))
        .for_each(q!(| _ | {}));
}
