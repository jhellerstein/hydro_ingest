use hydro_lang::*;

pub fn counter_hydro(process: &Process) {
    process
        .source_iter(q!(1..=5))
        .for_each(q!(|i| println!("Count: {}", i)));
}
