use hydro_lang::*;

pub fn hello_world_hydro(process: &Process) {
    process
        .source_iter(q!(std::iter::once("Hello, world!")))
        .for_each(q!(|msg| println!("{}", msg)));
}
